use crate::core::QueryHistory;
use crate::ui::styles::{DANGER, GRAY, SUCCESS};
use egui::{self, RichText};

pub struct HistoryPanel;

impl HistoryPanel {
    pub fn show(
        ctx: &egui::Context,
        show: &mut bool,
        history: &QueryHistory,
        selected_sql: &mut Option<String>,
        clear_history: &mut bool,
    ) {
        if !*show {
            return;
        }

        egui::Window::new("查询历史")
            .collapsible(true)
            .resizable(true)
            .default_size([500.0, 400.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("{} 条记录", history.len()));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("关闭 [Esc]").clicked() {
                            *show = false;
                        }
                        if ui
                            .add_enabled(!history.is_empty(), egui::Button::new("清空 [Ctrl+Del]"))
                            .clicked()
                        {
                            *clear_history = true;
                        }
                    });
                });

                ui.separator();

                if history.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.label(RichText::new("暂无查询历史").italics().color(GRAY));
                    });
                    return;
                }

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (idx, item) in history.items().iter().enumerate() {
                        let frame = egui::Frame::none()
                            .inner_margin(8.0)
                            .rounding(4.0)
                            .fill(ui.visuals().extreme_bg_color);

                        frame.show(ui, |ui| {
                            ui.horizontal(|ui| {
                                // 状态图标 - 使用图标+文字双重指示，对色盲友好
                                if item.success {
                                    ui.colored_label(SUCCESS, "✓ 成功");
                                } else {
                                    ui.colored_label(DANGER, "✗ 失败");
                                }

                                ui.separator();

                                // 数据库类型
                                ui.label(RichText::new(&item.database_type).small());

                                ui.separator();

                                // 时间戳
                                ui.label(
                                    RichText::new(item.timestamp.format("%H:%M:%S").to_string())
                                        .small()
                                        .color(GRAY),
                                );

                                // 影响行数
                                if let Some(rows) = item.rows_affected {
                                    ui.separator();
                                    ui.label(RichText::new(format!("{} 行", rows)).small());
                                }
                            });

                            // SQL 预览
                            let sql_preview = if item.sql.len() > 100 {
                                format!("{}...", &item.sql[..100])
                            } else {
                                item.sql.clone()
                            };

                            ui.add_space(4.0);
                            let response = ui.add(
                                egui::Label::new(
                                    RichText::new(&sql_preview).monospace().size(12.0),
                                )
                                .sense(egui::Sense::click()),
                            );

                            if response.clicked() {
                                *selected_sql = Some(item.sql.clone());
                                *show = false;
                            }

                            if response.hovered() {
                                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                            }

                            response.on_hover_text("点击使用此查询");
                        });

                        if idx < history.len() - 1 {
                            ui.add_space(4.0);
                        }
                    }
                });
            });
    }
}
