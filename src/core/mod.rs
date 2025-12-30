//! 核心模块 - 包含配置、主题、语法高亮、历史记录、导出等核心功能

mod autocomplete;
mod config;
pub mod constants;
mod export;
mod formatter;
mod history;
mod keybindings;
mod notification;
mod progress;
mod session;
mod syntax;
mod theme;

pub use autocomplete::{AutoComplete, CompletionKind};
pub use config::AppConfig;
#[allow(unused_imports)] // parse_csv_line 等供测试使用
pub use export::{
    ExportFormat,
    CsvImportConfig, JsonImportConfig,
    import_csv_to_sql, import_json_to_sql,
    preview_csv, preview_json,
    parse_csv_line, sql_value_from_string, json_value_to_sql,
};
pub use formatter::format_sql;
pub use history::QueryHistory;
pub use notification::{Notification, NotificationLevel, NotificationManager};
#[allow(unused_imports)] // 公开 API，供外部使用
pub use progress::{ProgressManager, ProgressTask};
#[allow(unused_imports)] // 公开 API
pub use syntax::{clear_highlight_cache, highlight_sql, HighlightColors, SqlHighlighter};
pub use theme::{ThemeManager, ThemePreset};
#[allow(unused_imports)] // 公开 API，供未来使用
pub use keybindings::{Action, KeyBinding, KeyBindings, KeyCode, KeyModifiers};
#[allow(unused_imports)] // 公开 API，供未来使用
pub use session::{SessionManager, SessionState, TabState, WindowState};
