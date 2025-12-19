# Gridix

> 当 Vim 用户遇到数据库，会发生什么？

简洁、快速、安全的跨平台数据库管理工具。**不用鼠标，照样起飞。**

![Version](https://img.shields.io/badge/version-0.5.1-green.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)
[![AUR](https://img.shields.io/aur/version/gridix-bin?label=AUR)](https://aur.archlinux.org/packages/gridix-bin)

```
Gridix = Grid + Helix = 表格数据 + 键盘流操作
```

## 截图

![Screenshot](gridix.png)

## 特性

- **多数据库** - SQLite / PostgreSQL / MySQL
- **安全连接** - SSH 隧道、SSL/TLS、AES-256 加密存储密码
- **键盘流** - Helix/Vim 风格，`hjkl` 导航，三种编辑模式
- **智能编辑** - 语法高亮、200+ 自动补全、SQL 格式化
- **19 种主题** - Tokyo Night / Catppuccin / Gruvbox / Dracula...
- **高级筛选** - 16 种操作符，支持正则和组合条件
- **轻量高效** - 纯 Rust，单文件 ~22MB，秒启动

## 安装

### Arch Linux (AUR)

```bash
paru -S gridix-bin        # 预编译版（推荐）
paru -S gridix-appimage   # AppImage 版
paru -S gridix            # 源码编译
```

### 下载预编译版本

[Releases 页面](https://github.com/MCB-SMART-BOY/gridix/releases) 提供：

- `gridix-linux-x86_64.tar.gz` / `gridix.AppImage`
- `gridix-windows-x86_64.zip`
- `gridix-macos-arm64.tar.gz` / `gridix-macos-x86_64.tar.gz`

### 从源码编译

```bash
git clone https://github.com/MCB-SMART-BOY/gridix.git
cd gridix && cargo build --release
```

<details>
<summary>Linux 依赖</summary>

```bash
# Debian/Ubuntu
sudo apt-get install libgtk-3-dev libxdo-dev

# Fedora
sudo dnf install gtk3-devel libxdo-devel

# Arch
sudo pacman -S gtk3 xdotool
```
</details>

## 快捷键

### 全局

| 键 | 功能 | 键 | 功能 |
|----|------|----|------|
| `Ctrl+N` | 新建连接 | `Ctrl+E` | 导出数据 |
| `Ctrl+Enter` | 执行 SQL | `Ctrl+I` | 导入数据 |
| `Ctrl+B` | 侧边栏 | `Ctrl+D` | 日/夜模式 |
| `Ctrl+J` | SQL 编辑器 | `Ctrl+T` | 主题选择 |
| `Ctrl+H` | 查询历史 | `F5` | 刷新 |

### 表格导航 (Normal 模式)

```
     k              gg → 首行    G → 末行
   h   l            gh → 行首    gl → 行尾
     j              Ctrl+u/d → 翻页    5j → 下移5行
```

### 表格编辑

| 键 | 功能 | 键 | 功能 |
|----|------|----|------|
| `i/c` | 编辑 | `y/p` | 复制/粘贴 |
| `d` | 删除 | `u` | 撤销 |
| `o/O` | 插入行 | `v/x` | 选择 |
| `Space+d` | 删行 | `Esc` | 退出编辑 |

### 筛选

| 键 | 功能 |
|----|------|
| `/` | 快速筛选 |
| `Ctrl+F` | 添加条件 |
| `Ctrl+Shift+F` | 清空 |

## 配置文件

| 系统 | 路径 |
|------|------|
| Linux | `~/.config/gridix/config.toml` |
| macOS | `~/Library/Application Support/gridix/config.toml` |
| Windows | `%APPDATA%\gridix\config.toml` |

## FAQ

<details>
<summary>Q: 支持 Oracle/SQL Server 吗？</summary>
暂不支持，专注 SQLite/PostgreSQL/MySQL。
</details>

<details>
<summary>Q: 密码安全吗？</summary>
使用 AES-256-GCM 加密存储。
</details>

## 更新日志

- **v0.5.1** - AUR 包、AppImage
- **v0.5.0** - Helix 键盘完整支持、列宽自适应
- **v0.4.0** - 对话框键盘导航、CI/CD
- **v0.3.0** - 侧边栏导航、数据导入
- **v0.2.0** - SSH 隧道、MySQL SSL
- **v0.1.0** - 初始版本

## 贡献

[Issues](https://github.com/MCB-SMART-BOY/gridix/issues) · [Pull Requests](https://github.com/MCB-SMART-BOY/gridix/pulls)

## 许可证

MIT

---

*献给所有被鼠标折磨过的键盘侠们。* ⌨️
