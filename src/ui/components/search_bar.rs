use crate::core::constants;
use crate::ui::styles::GRAY;
use egui::{self, Color32, RichText, TextEdit};

pub struct SearchBar;

impl SearchBar {
    pub fn show(
        ui: &mut egui::Ui,
        search_text: &mut String,
        search_column: &mut Option<String>,
        available_columns: &[String],
        result_count: Option<(usize, usize)>, // (filtered, total)
    ) {
        ui.horizontal(|ui| {
            ui.add_space(4.0);
            ui.label("搜索:");

            // 搜索输入框
            ui.add(
                TextEdit::singleline(search_text)
                    .hint_text("输入关键词搜索...")
                    .desired_width(200.0),
            );

            // 列选择
            if !available_columns.is_empty() {
                ui.separator();
                ui.label("列:");

                let selected_text = search_column.as_deref().unwrap_or("全部列");
                egui::ComboBox::new("search_column", "")
                    .selected_text(selected_text)
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(search_column.is_none(), "全部列")
                            .clicked()
                        {
                            *search_column = None;
                        }
                        for col in available_columns {
                            if ui
                                .selectable_label(search_column.as_deref() == Some(col), col)
                                .clicked()
                            {
                                *search_column = Some(col.clone());
                            }
                        }
                    });
            }

            // 清空搜索
            if !search_text.is_empty() && ui.button("✕ [Ctrl+K]").clicked() {
                search_text.clear();
            }

            // 结果统计
            if let Some((filtered, total)) = result_count {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 大结果集警告
                    let is_large = total >= constants::database::LARGE_RESULT_SET_WARNING_THRESHOLD;
                    let warning_color = Color32::from_rgb(255, 180, 80);
                    
                    if is_large {
                        ui.label(
                            RichText::new("⚠ 大数据集")
                                .small()
                                .color(warning_color),
                        ).on_hover_text(format!(
                            "结果集超过 {} 行，可能影响性能。\n建议添加 LIMIT 或 WHERE 条件缩小范围。",
                            constants::database::LARGE_RESULT_SET_WARNING_THRESHOLD
                        ));
                        ui.add_space(8.0);
                    }
                    
                    if filtered == total {
                        let color = if is_large { warning_color } else { GRAY };
                        ui.label(
                            RichText::new(format!("共 {} 行", total))
                                .small()
                                .color(color),
                        );
                    } else {
                        ui.label(
                            RichText::new(format!("显示 {} / {} 行", filtered, total))
                                .small()
                                .color(GRAY),
                        );
                    }
                });
            }
        });
    }
}
