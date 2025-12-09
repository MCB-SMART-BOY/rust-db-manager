//! 核心模块 - 包含配置、主题、语法高亮、历史记录、导出等核心功能

mod autocomplete;
mod config;
pub mod constants;
mod export;
mod formatter;
mod history;
mod syntax;
mod theme;

pub use autocomplete::{AutoComplete, CompletionKind};
pub use config::AppConfig;
pub use export::{export_to_csv, export_to_json, export_to_sql, import_sql_file, ExportFormat};
pub use formatter::format_sql;
pub use history::QueryHistory;
pub use syntax::{highlight_sql, HighlightColors};
pub use theme::{ThemeManager, ThemePreset};
