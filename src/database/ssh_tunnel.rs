//! SSH 隧道模块
//!
//! 提供 SSH 隧道功能，允许通过 SSH 跳板机连接远程数据库。

use async_trait::async_trait;
use russh::client::{Config, Handle, Handler};
use russh_keys::key::PublicKey;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, RwLock};

// ============================================================================
// 错误类型
// ============================================================================

#[derive(Error, Debug)]
pub enum SshError {
    #[error("SSH 连接失败: {0}")]
    Connection(String),
    #[error("SSH 认证失败: {0}")]
    Authentication(String),
    #[error("隧道创建失败: {0}")]
    Tunnel(String),
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    #[error("密钥加载失败: {0}")]
    Key(String),
}

// ============================================================================
// SSH 隧道配置
// ============================================================================

/// SSH 认证方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, Hash)]
pub enum SshAuthMethod {
    /// 密码认证
    #[default]
    Password,
    /// 私钥认证
    PrivateKey,
}

impl SshAuthMethod {
    /// 获取认证方式的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Password => "密码",
            Self::PrivateKey => "私钥",
        }
    }
    
    /// 获取所有认证方式
    #[allow(dead_code)] // 公开 API，供外部使用
    pub fn all() -> Vec<Self> {
        vec![Self::Password, Self::PrivateKey]
    }
}

/// SSH 隧道配置
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SshTunnelConfig {
    /// 是否启用 SSH 隧道
    pub enabled: bool,
    /// SSH 服务器地址
    pub ssh_host: String,
    /// SSH 服务器端口
    pub ssh_port: u16,
    /// SSH 用户名
    pub ssh_username: String,
    /// SSH 密码（密码认证时使用）
    pub ssh_password: String,
    /// 私钥路径（私钥认证时使用）
    pub private_key_path: String,
    /// 私钥密码（如果私钥有密码保护）
    pub private_key_passphrase: String,
    /// 认证方式
    pub auth_method: SshAuthMethod,
    /// 远程数据库主机（从 SSH 服务器视角）
    pub remote_host: String,
    /// 远程数据库端口
    pub remote_port: u16,
    /// 本地绑定端口（0 表示自动分配）
    pub local_port: u16,
}

#[allow(dead_code)] // 公开 API，供外部使用
impl SshTunnelConfig {
    /// 创建新的 SSH 隧道配置
    pub fn new() -> Self {
        Self {
            ssh_port: 22,
            ..Default::default()
        }
    }

    /// 获取 SSH 服务器地址
    pub fn ssh_addr(&self) -> String {
        format!("{}:{}", self.ssh_host, self.ssh_port)
    }

    /// 验证配置是否有效
    pub fn validate(&self) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        if self.ssh_host.is_empty() {
            return Err("SSH 主机地址不能为空".to_string());
        }

        if self.ssh_username.is_empty() {
            return Err("SSH 用户名不能为空".to_string());
        }

        if self.remote_host.is_empty() {
            return Err("远程数据库主机不能为空".to_string());
        }

        if self.remote_port == 0 {
            return Err("远程数据库端口无效".to_string());
        }

        match self.auth_method {
            SshAuthMethod::Password => {
                if self.ssh_password.is_empty() {
                    return Err("SSH 密码不能为空".to_string());
                }
            }
            SshAuthMethod::PrivateKey => {
                if self.private_key_path.is_empty() {
                    return Err("私钥路径不能为空".to_string());
                }
                if !std::path::Path::new(&self.private_key_path).exists() {
                    return Err("私钥文件不存在".to_string());
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// SSH 客户端处理器
// ============================================================================

struct SshClientHandler;

#[async_trait]
impl Handler for SshClientHandler {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &PublicKey,
    ) -> Result<bool, Self::Error> {
        // 在生产环境中，应该验证服务器密钥
        // 这里简化处理，始终接受
        Ok(true)
    }
}

// ============================================================================
// SSH 隧道
// ============================================================================

/// SSH 隧道状态
pub struct SshTunnel {
    /// 本地监听地址
    local_addr: SocketAddr,
    /// 是否正在运行
    running: Arc<RwLock<bool>>,
    /// 隧道任务句柄（保持任务存活，防止被丢弃）
    #[allow(dead_code)] // 字段用于保持任务生命周期，不需要直接访问
    task_handle: Option<tokio::task::JoinHandle<()>>,
}

impl SshTunnel {
    /// 创建并启动 SSH 隧道
    pub async fn start(config: &SshTunnelConfig) -> Result<Self, SshError> {
        // 验证配置
        config.validate().map_err(SshError::Connection)?;

        // 创建本地监听器
        let local_addr = format!("127.0.0.1:{}", config.local_port);
        let listener = TcpListener::bind(&local_addr).await?;
        let actual_local_addr = listener.local_addr()?;

        // 建立 SSH 连接
        let ssh_handle = Self::connect_ssh(config).await?;
        let ssh_handle = Arc::new(Mutex::new(ssh_handle));

        let running = Arc::new(RwLock::new(true));
        let running_clone = running.clone();
        let remote_host = config.remote_host.clone();
        let remote_port = config.remote_port;

        // 启动隧道转发任务
        let task_handle = tokio::spawn(async move {
            Self::run_tunnel(listener, ssh_handle, remote_host, remote_port, running_clone).await;
        });

        Ok(Self {
            local_addr: actual_local_addr,
            running,
            task_handle: Some(task_handle),
        })
    }

    /// 建立 SSH 连接
    async fn connect_ssh(config: &SshTunnelConfig) -> Result<Handle<SshClientHandler>, SshError> {
        let ssh_config = Config::default();
        let ssh_config = Arc::new(ssh_config);

        // 解析地址
        let addr: SocketAddr = config
            .ssh_addr()
            .parse()
            .map_err(|_| SshError::Connection("无效的 SSH 地址".to_string()))?;

        // 连接 SSH 服务器
        let mut session = russh::client::connect(ssh_config, addr, SshClientHandler)
            .await
            .map_err(|e| SshError::Connection(format!("连接失败: {}", e)))?;

        // 认证
        let authenticated = match config.auth_method {
            SshAuthMethod::Password => session
                .authenticate_password(&config.ssh_username, &config.ssh_password)
                .await
                .map_err(|e| SshError::Authentication(format!("密码认证失败: {}", e)))?,
            SshAuthMethod::PrivateKey => {
                let key_pair = russh_keys::load_secret_key(
                    &config.private_key_path,
                    if config.private_key_passphrase.is_empty() {
                        None
                    } else {
                        Some(&config.private_key_passphrase)
                    },
                )
                .map_err(|e| SshError::Key(format!("加载私钥失败: {}", e)))?;

                session
                    .authenticate_publickey(&config.ssh_username, Arc::new(key_pair))
                    .await
                    .map_err(|e| SshError::Authentication(format!("私钥认证失败: {}", e)))?
            }
        };

        if !authenticated {
            return Err(SshError::Authentication("认证失败".to_string()));
        }

        Ok(session)
    }

    /// 运行隧道转发
    async fn run_tunnel(
        listener: TcpListener,
        ssh_handle: Arc<Mutex<Handle<SshClientHandler>>>,
        remote_host: String,
        remote_port: u16,
        running: Arc<RwLock<bool>>,
    ) {
        while *running.read().await {
            tokio::select! {
                accept_result = listener.accept() => {
                    match accept_result {
                        Ok((local_stream, _)) => {
                            let ssh_handle = ssh_handle.clone();
                            let remote_host = remote_host.clone();

                            tokio::spawn(async move {
                                if let Err(e) = Self::forward_connection(
                                    local_stream,
                                    ssh_handle,
                                    &remote_host,
                                    remote_port,
                                ).await {
                                    eprintln!("[SSH Tunnel] 转发错误: {}", e);
                                }
                            });
                        }
                        Err(e) => {
                            eprintln!("[SSH Tunnel] 接受连接错误: {}", e);
                        }
                    }
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
                    // 检查是否应该停止
                }
            }
        }
    }

    /// 转发单个连接
    async fn forward_connection(
        mut local_stream: TcpStream,
        ssh_handle: Arc<Mutex<Handle<SshClientHandler>>>,
        remote_host: &str,
        remote_port: u16,
    ) -> Result<(), SshError> {
        // 通过 SSH 创建到远程主机的通道
        let channel = {
            let handle = ssh_handle.lock().await;
            handle
                .channel_open_direct_tcpip(remote_host, remote_port as u32, "127.0.0.1", 0)
                .await
                .map_err(|e| SshError::Tunnel(format!("创建通道失败: {}", e)))?
        };

        // 双向转发数据
        let (mut local_read, mut local_write) = local_stream.split();
        let mut channel_stream = channel.into_stream();
        let (mut channel_read, mut channel_write) = tokio::io::split(&mut channel_stream);

        let local_to_remote = async {
            let mut buf = [0u8; 8192];
            loop {
                match local_read.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        if channel_write.write_all(&buf[..n]).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        };

        let remote_to_local = async {
            let mut buf = [0u8; 8192];
            loop {
                match channel_read.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        if local_write.write_all(&buf[..n]).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        };

        tokio::select! {
            _ = local_to_remote => {}
            _ = remote_to_local => {}
        }

        Ok(())
    }

    /// 获取本地监听地址
    #[allow(dead_code)] // 公开 API，供外部使用
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    /// 获取本地端口
    pub fn local_port(&self) -> u16 {
        self.local_addr.port()
    }

    /// 停止隧道
    pub async fn stop(&self) {
        let mut running = self.running.write().await;
        *running = false;
    }

    /// 检查隧道是否正在运行
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
}

// ============================================================================
// SSH 隧道管理器
// ============================================================================

/// SSH 隧道管理器
pub struct SshTunnelManager {
    tunnels: RwLock<std::collections::HashMap<String, Arc<SshTunnel>>>,
}

impl SshTunnelManager {
    /// 创建新的隧道管理器
    pub fn new() -> Self {
        Self {
            tunnels: RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// 创建或获取隧道
    pub async fn get_or_create(
        &self,
        name: &str,
        config: &SshTunnelConfig,
    ) -> Result<Arc<SshTunnel>, SshError> {
        // 检查是否已有隧道
        {
            let tunnels = self.tunnels.read().await;
            if let Some(tunnel) = tunnels.get(name) {
                if tunnel.is_running().await {
                    return Ok(tunnel.clone());
                }
            }
        }

        // 创建新隧道
        let tunnel = Arc::new(SshTunnel::start(config).await?);

        {
            let mut tunnels = self.tunnels.write().await;
            tunnels.insert(name.to_string(), tunnel.clone());
        }

        Ok(tunnel)
    }

    /// 停止指定隧道
    pub async fn stop(&self, name: &str) {
        let tunnel = {
            let mut tunnels = self.tunnels.write().await;
            tunnels.remove(name)
        };

        if let Some(tunnel) = tunnel {
            tunnel.stop().await;
        }
    }

    /// 停止所有隧道
    #[allow(dead_code)] // 公开 API，供外部使用
    pub async fn stop_all(&self) {
        let tunnels: Vec<_> = {
            let mut tunnels = self.tunnels.write().await;
            tunnels.drain().map(|(_, t)| t).collect()
        };

        for tunnel in tunnels {
            tunnel.stop().await;
        }
    }
}

impl Default for SshTunnelManager {
    fn default() -> Self {
        Self::new()
    }
}

// 全局隧道管理器
lazy_static::lazy_static! {
    pub static ref SSH_TUNNEL_MANAGER: SshTunnelManager = SshTunnelManager::new();
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation_disabled() {
        let config = SshTunnelConfig {
            enabled: false,
            ..Default::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_missing_host() {
        let config = SshTunnelConfig {
            enabled: true,
            ssh_host: String::new(),
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_password() {
        let config = SshTunnelConfig {
            enabled: true,
            ssh_host: "example.com".to_string(),
            ssh_port: 22,
            ssh_username: "user".to_string(),
            auth_method: SshAuthMethod::Password,
            ssh_password: "pass".to_string(),
            remote_host: "localhost".to_string(),
            remote_port: 3306,
            ..Default::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_auth_method_display() {
        assert_eq!(SshAuthMethod::Password.display_name(), "密码");
        assert_eq!(SshAuthMethod::PrivateKey.display_name(), "私钥");
    }
}
