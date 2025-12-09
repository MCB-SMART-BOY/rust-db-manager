# Rust DB Manager

简洁、快速、安全的跨平台数据库管理工具。

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)

## 功能特性

- **多数据库支持** - SQLite、PostgreSQL、MySQL
- **SQL 编辑器** - 语法高亮、自动补全、SQL 格式化
- **数据浏览** - 表格展示、排序、筛选、搜索
- **数据导出** - 支持 CSV、JSON、SQL 格式导出
- **查询历史** - 自动保存查询历史，快速复用
- **主题切换** - 19 种主题预设，支持明暗模式切换
- **连接管理** - 多连接管理，密码加密存储
- **键盘友好** - 丰富的快捷键支持

## 截图

![Screenshot](rust-db-manager.png)

## 安装

### 下载预编译版本

从 [Releases](https://github.com/MCB-SMART-BOY/rust-db-manager/releases) 页面下载对应平台的版本：

| 平台 | 下载 |
|------|------|
| Linux x86_64 | `rust-db-manager-linux-x86_64.tar.gz` (含 AppImage) |
| Windows x86_64 | `rust-db-manager-windows-x86_64.zip` |
| macOS Apple Silicon | `rust-db-manager-macos-arm64.tar.gz` |
| macOS Intel | `rust-db-manager-macos-x86_64.tar.gz` |

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/MCB-SMART-BOY/rust-db-manager.git
cd rust-db-manager

# 编译运行
cargo run --release
```

#### 依赖项

**Linux:**
```bash
sudo apt-get install libgtk-3-dev libxdo-dev
```

**macOS / Windows:** 无额外依赖

## 快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+N` | 新建连接 |
| `Ctrl+Enter` | 执行查询 |
| `Ctrl+J` | 切换 SQL 编辑器 |
| `Ctrl+B` | 切换侧边栏 |
| `Ctrl+H` | 查询历史 |
| `Ctrl+E` | 导出结果 |
| `Ctrl+I` | 导入 SQL |
| `Ctrl+D` | 切换明暗模式 |
| `Ctrl+T` | 主题选择 |
| `Ctrl+F` | 添加筛选条件 |
| `Ctrl++/-` | 缩放界面 |
| `F5` | 刷新表列表 |
| `F1` | 帮助 |

## 使用说明

### 连接数据库

1. 点击 `+ 新建` 或按 `Ctrl+N`
2. 选择数据库类型（SQLite / PostgreSQL / MySQL）
3. 填写连接信息
4. 点击保存

### SQLite
- 直接选择 `.db` 或 `.sqlite` 文件

### PostgreSQL / MySQL
- 填写主机、端口、用户名、密码
- 连接后选择数据库

### 执行查询

1. 在 SQL 编辑器中输入查询语句
2. 按 `Ctrl+Enter` 执行
3. 结果显示在数据表格中

### 导出数据

1. 执行查询后，按 `Ctrl+E`
2. 选择导出格式（CSV / JSON / SQL）
3. 选择保存位置

## 技术栈

- **GUI 框架**: [egui](https://github.com/emilk/egui) / [eframe](https://github.com/emilk/egui/tree/master/crates/eframe)
- **异步运行时**: [Tokio](https://tokio.rs/)
- **数据库驱动**:
  - SQLite: [rusqlite](https://github.com/rusqlite/rusqlite)
  - PostgreSQL: [tokio-postgres](https://github.com/sfackler/rust-postgres)
  - MySQL: [mysql_async](https://github.com/blackbeam/mysql_async)

## 开发

```bash
# 开发模式运行
cargo run

# 运行测试
cargo test

# 构建 Release 版本
cargo build --release

# 构建 AppImage (Linux)
cargo appimage
```

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 贡献

欢迎提交 Issue 和 Pull Request！
