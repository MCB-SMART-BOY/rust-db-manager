# Gridix

> 当 Vim 用户遇到数据库，会发生什么？

简洁、快速、安全的跨平台数据库管理工具。**不用鼠标，照样起飞。**

![Version](https://img.shields.io/badge/version-0.5.1-green.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)
[![AUR](https://img.shields.io/aur/version/gridix-bin?label=AUR)](https://aur.archlinux.org/packages/gridix-bin)

```
Gridix = Grid + Helix
       = 表格数据 + 键盘流操作
       = 你的新生产力工具
```

## 为什么选择 Gridix？

| 其他工具 | Gridix |
|---------|--------|
| 点点点点点... | `hjkl` 走天下 |
| 找菜单找半天 | `Ctrl+Enter` 执行，完事 |
| 连接数据库要点 N 个按钮 | `Ctrl+N`，填完回车，搞定 |
| 导出数据要等加载... | 异步执行，界面不卡 |
| 密码明文存配置文件 | AES-256-GCM 加密，安全感拉满 |
| Electron 套壳，吃内存 | 纯 Rust，轻量高效 |
| 启动要等好几秒 | 秒开，不废话 |

## 截图

![Screenshot](gridix.png)

*看到右边那个表格了吗？用 `hjkl` 就能导航，用 `c` 就能改内容。是的，就像 Vim 一样。*

## 特性一览

```
+------------------+     +------------------+     +------------------+
|   数据库支持      |     |    安全连接       |     |   键盘流操作      |
+------------------+     +------------------+     +------------------+
| - SQLite         |     | - SSH 隧道       |     | - Helix/Vim 键位 |
| - PostgreSQL     |     | - MySQL SSL/TLS  |     | - hjkl 导航      |
| - MySQL/MariaDB  |     | - AES-256 加密   |     | - 三种编辑模式    |
+------------------+     +------------------+     +------------------+

+------------------+     +------------------+     +------------------+
|   智能编辑器      |     |    主题系统       |     |   数据操作        |
+------------------+     +------------------+     +------------------+
| - 语法高亮       |     | - 19 种预设      |     | - 导入 CSV/JSON  |
| - 200+ 自动补全  |     | - 日/夜模式      |     | - 导出 CSV/JSON  |
| - SQL 格式化     |     | - 自定义配色     |     | - 16 种筛选器    |
+------------------+     +------------------+     +------------------+
```

## 安装

### Arch Linux (AUR) - 推荐

Arch 用户福利，一行命令：

```bash
# 预编译版（推荐，秒装）
paru -S gridix-bin

# AppImage 版（自带依赖，专治依赖冲突）
paru -S gridix-appimage

# 源码编译版（硬核玩家）
paru -S gridix
```

> 用 `yay`？把 `paru` 换成 `yay` 就行。

### 下载预编译版本

从 [Releases](https://github.com/MCB-SMART-BOY/gridix/releases) 下载：

| 平台 | 文件 | 大小 | 说明 |
|------|------|------|------|
| Linux | `gridix-linux-x86_64.tar.gz` | ~13 MB | 通用版本 |
| Linux | `gridix.AppImage` | ~17 MB | 双击即用 |
| Windows | `gridix-windows-x86_64.zip` | ~12 MB | 解压运行 |
| macOS (ARM) | `gridix-macos-arm64.tar.gz` | ~12 MB | M1/M2/M3/M4 |
| macOS (Intel) | `gridix-macos-x86_64.tar.gz` | ~12 MB | 老款 Mac |

### 从源码编译

```bash
git clone https://github.com/MCB-SMART-BOY/gridix.git
cd gridix
cargo build --release
# 产物在 target/release/gridix
```

<details>
<summary>Linux 依赖安装</summary>

```bash
# Debian/Ubuntu
sudo apt-get install libgtk-3-dev libxdo-dev

# Fedora
sudo dnf install gtk3-devel libxdo-devel

# Arch Linux
sudo pacman -S gtk3 xdotool

# openSUSE
sudo zypper install gtk3-devel libxdo-devel
```
</details>

## 快速上手

```
1. Ctrl+N        → 新建连接
2. 选择数据库类型  → 填写连接信息 → 回车
3. 侧边栏选表     → 数据自动加载
4. hjkl 导航     → c 修改 → Ctrl+S 保存
```

就这么简单。

## 快捷键

### 全局快捷键

| 快捷键 | 功能 | | 快捷键 | 功能 |
|--------|------|-|--------|------|
| `Ctrl+N` | 新建连接 | | `Ctrl+E` | 导出数据 |
| `Ctrl+Enter` | 执行 SQL | | `Ctrl+I` | 导入数据 |
| `Ctrl+B` | 切换侧边栏 | | `Ctrl+D` | 日/夜模式 |
| `Ctrl+J` | 切换编辑器 | | `Ctrl+T` | 主题选择 |
| `Ctrl+H` | 查询历史 | | `F5` | 刷新列表 |

### 表格操作（Helix/Vim 风格）

**导航：**
```
     k
   h   l      gg → 首行    G → 末行
     j        gh → 行首    gl → 行尾
              Ctrl+u/d → 翻半页
              5j → 向下5行（数字前缀）
```

**编辑：**
| 键 | 功能 | | 键 | 功能 |
|----|------|-|----|------|
| `i` | 插入模式 | | `y` | 复制 |
| `c` | 修改单元格 | | `p` | 粘贴 |
| `d` | 删除内容 | | `u` | 撤销 |
| `o` | 下方插入行 | | `O` | 上方插入行 |
| `v` | 选择模式 | | `x` | 选择整行 |
| `Space+d` | 删除整行 | | `Esc` | 返回 Normal |

## 高级功能

### 筛选系统

16 种操作符，支持组合：

```
文本: Contains / NotContains / Equals / StartsWith / EndsWith / Regex
数值: GreaterThan / LessThan / GreaterOrEqual / LessOrEqual / Between
空值: Empty / NotEmpty / IsNull / IsNotNull
逻辑: AND / OR 组合
```

| 快捷键 | 功能 |
|--------|------|
| `/` | 快速筛选 |
| `Ctrl+F` | 添加筛选条件 |
| `Ctrl+Shift+F` | 清空筛选 |

### SSH 隧道

连接远程数据库？通过跳板机？没问题：

```
本地 → SSH 服务器 → 远程数据库
      (密码/密钥)
```

支持密码认证和私钥认证。

### MySQL SSL/TLS

5 种安全模式：

| 模式 | 说明 |
|------|------|
| Disabled | 不加密 |
| Preferred | 优先加密 |
| Required | 强制加密 |
| VerifyCa | 验证 CA |
| VerifyIdentity | 验证证书+主机名 |

## 主题

19 种预设，日夜独立配置：

**暗色系：**
`Tokyo Night` / `Catppuccin Mocha` / `One Dark` / `Gruvbox Dark` / `Dracula` / `Nord` / `Monokai Pro` / `GitHub Dark`...

**亮色系：**
`Tokyo Night Light` / `Catppuccin Latte` / `One Light` / `Gruvbox Light` / `GitHub Light`...

`Ctrl+D` 切换日夜，`Ctrl+T` 选主题。

## 配置文件

| 系统 | 路径 |
|------|------|
| Linux | `~/.config/gridix/config.toml` |
| macOS | `~/Library/Application Support/gridix/config.toml` |
| Windows | `%APPDATA%\gridix\config.toml` |

配置自动保存，包括：连接信息（密码加密）、主题设置、窗口位置、查询历史。

## 技术栈

| 组件 | 技术 | 为什么选它 |
|------|------|-----------|
| GUI | egui 0.29 | 即时模式，跨平台，GPU 加速 |
| 异步 | Tokio | Rust 异步标准 |
| SQLite | rusqlite | 官方绑定，零配置 |
| PostgreSQL | tokio-postgres | 纯 Rust，异步原生 |
| MySQL | mysql_async | 纯 Rust，异步原生 |
| SSH | russh | 纯 Rust，安全可靠 |
| 加密 | ring | 工业级加密库 |
| 语法高亮 | syntect | VS Code 同款引擎 |

```
代码量: ~10,000 行 Rust
二进制: ~22 MB (单文件，无依赖)
内存占用: ~50 MB (取决于数据量)
启动时间: <1 秒
```

## 常见问题

<details>
<summary>Q: 为什么选择 Helix 风格而不是纯 Vim？</summary>

Helix 的选择-操作模式更直观：先选择，再操作。而且 Helix 是 Rust 写的，气质相投。
</details>

<details>
<summary>Q: 支持 Oracle/SQL Server 吗？</summary>

暂不支持。专注做好 SQLite/PostgreSQL/MySQL 三件套。
</details>

<details>
<summary>Q: 为什么不用 Tauri/Electron？</summary>

因为我们是 Rust 纯血党。egui 虽然不如 Web 技术栈好看，但够快够轻。
</details>

<details>
<summary>Q: 密码安全吗？</summary>

使用 AES-256-GCM 加密存储，密钥派生自系统特征。比明文强多了。
</details>

## 更新日志

### v0.5.1 (2025-12-20)
- 新增 AUR 包（gridix-bin / gridix / gridix-appimage）
- 新增 AppImage 打包
- 修复 GitHub Actions macOS 构建

### v0.5.0 (2025-12-20)
- 新增 Helix 键盘操作完整支持
- 新增历史面板键盘导航
- 数据表格列宽智能自适应
- 统一操作逻辑

### v0.4.0 (2025-12-18)
- 对话框统一键盘导航
- GitHub Actions 跨平台自动构建

### v0.3.0 (2025-12-15)
- 侧边栏键盘导航
- 数据导入功能

### v0.2.0 (2025-12-10)
- MySQL SSL/TLS
- SSH 隧道
- 多标签查询

### v0.1.0 (2024-12-09)
- 初始版本

## 参与贡献

```bash
# 克隆
git clone https://github.com/MCB-SMART-BOY/gridix.git

# 开发运行
cargo run

# 测试
cargo test

# 代码检查
cargo clippy
```

有问题？有想法？

- [提 Issue](https://github.com/MCB-SMART-BOY/gridix/issues) - Bug 报告、功能建议
- [发 PR](https://github.com/MCB-SMART-BOY/gridix/pulls) - 代码贡献
- [讨论区](https://github.com/MCB-SMART-BOY/gridix/discussions) - 闲聊吹水

## 致谢

感谢这些优秀的开源项目：

- [egui](https://github.com/emilk/egui) - 优雅的即时模式 GUI
- [Helix](https://helix-editor.com/) - 键位设计灵感
- [Catppuccin](https://github.com/catppuccin) - 舒适的配色方案
- [Tokyo Night](https://github.com/enkia/tokyo-night-vscode-theme) - 经典主题

## 许可证

MIT License - 随便用，改着玩，商用也行。

觉得好用？给个 Star ⭐ 就是最好的支持。

---

<p align="center">
<i>献给所有被鼠标折磨过的键盘侠们。</i>
<br><br>
<b>Happy Hacking! 🚀</b>
</p>
