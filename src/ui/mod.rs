//! UI 模块 - 用户界面组件

pub mod components;
pub mod dialogs;
pub mod panels;
pub mod styles;

// 重新导出常用组件
pub use components::{
    quote_identifier, DataGrid, DataGridState, SearchBar, SqlEditor, SqlEditorActions, Toolbar,
    ToolbarActions, Welcome,
    // 多 Tab 查询
    QueryTabBar, QueryTabManager,
};
pub use dialogs::{ConfirmDialog, ConnectionDialog, DdlDialog, DdlDialogState, ExportConfig, ExportDialog, HelpDialog};
pub use panels::{HistoryPanel, Sidebar, SidebarActions};
