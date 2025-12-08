//! 数据表格组件模块
//!
//! 提供 Helix 编辑器风格的模态操作数据表格。
//!
//! ## 模块结构
//! - `state`: 表格状态管理
//! - `mode`: 编辑模式定义
//! - `filter`: 筛选条件
//! - `keyboard`: 键盘输入处理
//! - `render`: 单元格渲染
//! - `actions`: 操作和 SQL 生成

#![allow(clippy::too_many_arguments)]

mod actions;
mod filter;
mod keyboard;
mod mode;
mod render;
mod state;

pub use actions::DataGridActions;
pub use filter::{ColumnFilter, FilterOperator};
pub use mode::GridMode;
pub use state::DataGridState;

use crate::database::QueryResult;
use crate::ui::styles::GRAY;
use egui::{self, RichText};
use egui_extras::{Column, TableBuilder};

// ============================================================================
// 常量定义
// ============================================================================

pub(crate) const TEXT_HEIGHT: f32 = 20.0;
pub(crate) const ROW_HEIGHT: f32 = TEXT_HEIGHT + 8.0;
pub(crate) const HEADER_HEIGHT: f32 = 28.0;
pub(crate) const MIN_COL_WIDTH: f32 = 80.0;
pub(crate) const ROW_NUM_WIDTH: f32 = 50.0;
pub(crate) const CELL_TRUNCATE_LEN: usize = 50;

use egui::Color32;
pub(crate) const COLOR_CELL_SELECTED: Color32 = Color32::from_rgb(60, 100, 180);
pub(crate) const COLOR_ROW_SELECTED: Color32 = Color32::from_rgb(50, 50, 70);
pub(crate) const COLOR_ROW_NUM_BG: Color32 = Color32::from_rgb(40, 40, 45);
pub(crate) const COLOR_CELL_EDITING: Color32 = Color32::from_rgb(80, 120, 200);
pub(crate) const COLOR_CELL_MODIFIED: Color32 = Color32::from_rgb(100, 150, 80);
pub(crate) const COLOR_VISUAL_SELECT: Color32 = Color32::from_rgb(120, 80, 160);

// ============================================================================
// 数据表格组件
// ============================================================================

pub struct DataGrid;

impl DataGrid {
    /// 显示可编辑的数据表格（Helix 风格）
    pub fn show_editable(
        ui: &mut egui::Ui,
        result: &QueryResult,
        search_text: &str,
        search_column: &Option<String>,
        selected_row: &mut Option<usize>,
        selected_cell: &mut Option<(usize, usize)>,
        state: &mut DataGridState,
        table_name: Option<&str>,
    ) -> (DataGridActions, (usize, usize)) {
        let mut actions = DataGridActions::default();

        if result.columns.is_empty() {
            Self::show_empty(ui);
            return (actions, (0, 0));
        }

        // 显示模式状态栏和操作按钮
        Self::show_mode_bar(ui, state, result, table_name, &mut actions);

        ui.add_space(2.0);

        // 显示跳转对话框
        Self::show_goto_dialog(ui.ctx(), state, result.rows.len());

        // 显示保存确认对话框
        Self::show_save_confirm_dialog(ui.ctx(), state, &mut actions);

        // 显示筛选栏
        filter::show_filter_bar(ui, result, state);

        ui.add_space(4.0);

        // 过滤行
        let filtered_rows = filter::filter_rows(result, search_text, search_column, &state.filters);
        let filtered_count = filtered_rows.len();
        let total_count = result.rows.len();

        // 处理键盘输入
        keyboard::handle_keyboard(ui, state, result, &filtered_rows, &mut actions);

        // 处理 Ctrl+S 保存请求
        if state.pending_save && state.has_changes() {
            if let Some(table) = table_name {
                actions::generate_save_sql(result, state, table, &mut actions);
            }
            state.pending_save = false;
        } else if state.pending_save {
            state.pending_save = false;
        }

        // 同步选择状态
        *selected_row = Some(state.cursor.0);
        *selected_cell = Some(state.cursor);

        // 计算列宽
        let available_width = ui.available_width();
        let data_width = available_width - ROW_NUM_WIDTH;
        let col_width = (data_width / result.columns.len() as f32).max(MIN_COL_WIDTH);

        // 收集需要添加筛选的列
        let mut columns_to_filter: Vec<String> = Vec::new();

        // 获取需要滚动到的行
        let scroll_to_row = state.scroll_to_row.take();

        // 创建表格
        let table_response = egui::Frame::none().show(ui, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    // 构建表格，使用 scroll_to_row 实现滚动跟随
                    let mut table_builder = TableBuilder::new(ui)
                        .striped(false) // 我们自己实现斑马线
                        .resizable(true)
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                        .column(Column::exact(ROW_NUM_WIDTH))
                        .columns(
                            Column::initial(col_width)
                                .at_least(MIN_COL_WIDTH)
                                .clip(true),
                            result.columns.len(),
                        );

                    // 如果需要滚动到某行
                    if let Some(target_row) = scroll_to_row {
                        table_builder =
                            table_builder.scroll_to_row(target_row, Some(egui::Align::Center));
                    }

                    table_builder
                        .header(HEADER_HEIGHT, |mut header| {
                            // 行号列头
                            header.col(|ui| {
                                ui.label(RichText::new("#").strong().color(GRAY));
                            });
                            // 数据列头
                            for (col_idx, col_name) in result.columns.iter().enumerate() {
                                header.col(|ui| {
                                    render::render_column_header(
                                        ui,
                                        col_name,
                                        col_idx,
                                        state,
                                        &mut columns_to_filter,
                                    );
                                });
                            }
                        })
                        .body(|body| {
                            body.rows(ROW_HEIGHT, filtered_count, |mut row| {
                                let display_idx = row.index();
                                if let Some((original_idx, row_data)) =
                                    filtered_rows.get(display_idx)
                                {
                                    let is_cursor_row = state.cursor.0 == *original_idx;
                                    let is_row_deleted =
                                        state.rows_to_delete.contains(original_idx);

                                    // 行号列
                                    row.col(|ui| {
                                        render::render_row_number(
                                            ui,
                                            *original_idx,
                                            is_cursor_row,
                                            is_row_deleted,
                                            state,
                                        );
                                    });

                                    // 数据列
                                    for (col_idx, cell) in row_data.iter().enumerate() {
                                        row.col(|ui| {
                                            render::render_editable_cell(
                                                ui,
                                                cell,
                                                *original_idx,
                                                col_idx,
                                                is_cursor_row,
                                                is_row_deleted,
                                                state,
                                            );
                                        });
                                    }
                                }
                            });
                        });
                });
        });

        // 处理列筛选点击
        for col_name in columns_to_filter {
            if !state.filters.iter().any(|f| f.column == col_name) {
                state.filters.push(ColumnFilter {
                    column: col_name,
                    operator: FilterOperator::Contains,
                    value: String::new(),
                });
            }
        }

        // 点击表格区域聚焦
        if table_response.response.clicked() {
            state.focused = true;
        }

        (actions, (filtered_count, total_count))
    }

    /// 显示模式状态栏和操作按钮
    fn show_mode_bar(
        ui: &mut egui::Ui,
        state: &mut DataGridState,
        result: &QueryResult,
        table_name: Option<&str>,
        actions: &mut DataGridActions,
    ) {
        ui.horizontal(|ui| {
            // 模式指示器
            let mode_text = format!("-- {} --", state.mode.display_name());
            ui.label(RichText::new(mode_text).strong().color(state.mode.color()));

            ui.separator();

            // 光标位置
            let pos_text = format!("{}:{}", state.cursor.0 + 1, state.cursor.1 + 1);
            ui.label(RichText::new(pos_text).monospace().color(GRAY));

            // 选择范围
            if let Some(((min_r, min_c), (max_r, max_c))) = state.get_selection() {
                let sel_text = format!("选择: {}x{}", max_r - min_r + 1, max_c - min_c + 1);
                ui.separator();
                ui.label(RichText::new(sel_text).small().color(COLOR_VISUAL_SELECT));
            }

            // 命令缓冲
            if !state.command_buffer.is_empty() {
                ui.separator();
                ui.label(
                    RichText::new(&state.command_buffer)
                        .monospace()
                        .color(Color32::YELLOW),
                );
            }

            // 计数
            if let Some(count) = state.count {
                ui.separator();
                ui.label(
                    RichText::new(format!("{}", count))
                        .monospace()
                        .color(Color32::YELLOW),
                );
            }

            ui.separator();

            // 筛选按钮
            if ui
                .button("+ 筛选 [/]")
                .on_hover_text("添加数据筛选条件\n快捷键: / (在 Normal 模式)")
                .clicked()
            {
                state.filters.push(ColumnFilter {
                    column: result.columns.first().cloned().unwrap_or_default(),
                    operator: FilterOperator::Contains,
                    value: String::new(),
                });
            }

            // 操作按钮
            if table_name.is_some() {
                if ui
                    .button("+ 行 [o]")
                    .on_hover_text("在表格末尾添加新行\n快捷键: o (在 Normal 模式)")
                    .clicked()
                {
                    let new_row = vec!["".to_string(); result.columns.len()];
                    state.new_rows.push(new_row);
                    actions.message = Some("已添加新行".to_string());
                }

                let has_changes = state.has_changes();
                if ui
                    .add_enabled(has_changes, egui::Button::new("保存 [:w]"))
                    .on_hover_text("保存所有修改到数据库\n快捷键: :w")
                    .clicked()
                {
                    if let Some(table) = table_name {
                        actions::generate_save_sql(result, state, table, actions);
                    }
                }

                if ui
                    .add_enabled(has_changes, egui::Button::new("放弃 [:q!]"))
                    .on_hover_text("放弃所有未保存的修改\n快捷键: :q!")
                    .clicked()
                {
                    state.clear_edits();
                    actions.message = Some("已放弃所有修改".to_string());
                }

                if has_changes {
                    ui.separator();
                    let mut stats = Vec::new();
                    if !state.modified_cells.is_empty() {
                        stats.push(format!("{}处修改", state.modified_cells.len()));
                    }
                    if !state.rows_to_delete.is_empty() {
                        stats.push(format!("{}行删除", state.rows_to_delete.len()));
                    }
                    if !state.new_rows.is_empty() {
                        stats.push(format!("{}行新增", state.new_rows.len()));
                    }
                    ui.label(
                        RichText::new(stats.join(", "))
                            .small()
                            .color(COLOR_CELL_MODIFIED),
                    );
                }
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let help = match state.mode {
                    GridMode::Normal => {
                        "hjkl:移动 i:编辑 v:选择 d:删除 y:复制 p:粘贴 gg:顶部 G:底部"
                    }
                    GridMode::Insert => "Esc:退出 Enter:确认",
                    GridMode::Select => "hjkl:扩展 d:删除 y:复制 Esc:取消",
                };
                ui.label(RichText::new(help).small().color(GRAY));
            });
        });
    }

    /// 显示空状态
    fn show_empty(ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.label(RichText::new("暂无数据").color(GRAY));
        });
    }

    /// 显示跳转对话框 (Ctrl+G)
    fn show_goto_dialog(ctx: &egui::Context, state: &mut DataGridState, max_row: usize) {
        if !state.show_goto_dialog {
            return;
        }

        egui::Window::new("跳转到行")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("行号:");
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut state.goto_input)
                            .desired_width(100.0)
                            .hint_text(format!("1-{}", max_row)),
                    );

                    // 自动聚焦
                    if response.gained_focus() || state.goto_input.is_empty() {
                        response.request_focus();
                    }

                    // 回车确认
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if let Ok(line) = state.goto_input.trim().parse::<usize>() {
                            if line >= 1 && line <= max_row {
                                state.cursor.0 = line - 1;
                                state.scroll_to_row = Some(state.cursor.0);
                            }
                        }
                        state.show_goto_dialog = false;
                        state.goto_input.clear();
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("跳转 [Enter]").clicked() {
                        if let Ok(line) = state.goto_input.trim().parse::<usize>() {
                            if line >= 1 && line <= max_row {
                                state.cursor.0 = line - 1;
                                state.scroll_to_row = Some(state.cursor.0);
                            }
                        }
                        state.show_goto_dialog = false;
                        state.goto_input.clear();
                    }
                    if ui.button("取消 [Esc]").clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape))
                    {
                        state.show_goto_dialog = false;
                        state.goto_input.clear();
                    }
                });
            });
    }

    /// 显示保存确认对话框（危险操作确认）
    fn show_save_confirm_dialog(
        ctx: &egui::Context,
        state: &mut DataGridState,
        actions: &mut DataGridActions,
    ) {
        if !state.show_save_confirm {
            return;
        }

        let delete_count = state.rows_to_delete.len();
        let total_count = state.pending_sql.len();

        egui::Window::new("⚠ 确认保存")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("此操作包含危险操作，请确认：").strong());
                    ui.add_space(8.0);

                    // 显示操作统计
                    ui.horizontal(|ui| {
                        ui.label(format!("• 将删除 {} 行数据", delete_count));
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("• 共 {} 条 SQL 语句", total_count));
                    });

                    ui.add_space(8.0);

                    // 显示预览的 SQL（最多显示5条）
                    ui.collapsing("查看 SQL 预览", |ui| {
                        egui::ScrollArea::vertical()
                            .max_height(150.0)
                            .show(ui, |ui| {
                                for (i, sql) in state.pending_sql.iter().enumerate() {
                                    let is_delete = sql.starts_with("DELETE");
                                    let color = if is_delete {
                                        Color32::from_rgb(200, 80, 80)
                                    } else {
                                        GRAY
                                    };
                                    ui.label(
                                        RichText::new(format!("{}. {}", i + 1, sql))
                                            .small()
                                            .color(color),
                                    );
                                }
                            });
                    });

                    ui.add_space(12.0);

                    ui.horizontal(|ui| {
                        // 确认按钮（红色警告）
                        if ui
                            .add(
                                egui::Button::new(RichText::new("确认执行 [Enter]").color(Color32::WHITE))
                                    .fill(Color32::from_rgb(180, 60, 60)),
                            )
                            .clicked()
                        {
                            actions::confirm_pending_sql(state, actions);
                        }

                        ui.add_space(16.0);

                        if ui.button("取消 [Esc]").clicked()
                            || ui.input(|i| i.key_pressed(egui::Key::Escape))
                        {
                            actions::cancel_pending_sql(state);
                        }
                    });
                });
            });
    }
}
