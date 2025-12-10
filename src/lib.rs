//! # Rust 数据库管理器
//!
//! 一个跨平台的数据库管理 GUI 工具，支持 SQLite、PostgreSQL 和 MySQL。
//!
//! ## 功能特性
//!
//! - 多数据库支持：SQLite、PostgreSQL、MySQL
//! - 多行 SQL 编辑器，支持语法高亮
//! - SQL 自动补全和格式化
//! - 查询结果导出 (CSV/SQL/JSON)
//! - 批量数据导入 (CSV/JSON)
//! - 19 种主题预设
//! - 查询历史记录
//! - 多 Tab 查询窗口
//! - SSH 隧道支持
//!
//! ## 模块结构
//!
//! - `core`: 核心功能（配置、主题、导出、语法高亮等）
//! - `database`: 数据库连接和查询
//! - `ui`: 用户界面组件

pub mod core;
pub mod database;
pub mod ui;
