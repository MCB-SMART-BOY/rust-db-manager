//! 确认对话框组件

use crate::ui::styles::{DANGER, SPACING_MD, SPACING_LG};
use egui::{self, Color32, RichText, Rounding};

pub struct ConfirmDialog;

impl ConfirmDialog {
    pub fn show(
        ctx: &egui::Context,
        show: &mut bool,
        title: &str,
        message: &str,
        confirm_text: &str,
        on_confirm: &mut bool,
    ) {
        if !*show {
            return;
        }

        egui::Window::new(title)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .min_width(320.0)
            .show(ctx, |ui| {
                ui.add_space(SPACING_MD);

                // 警告图标和消息
                ui.horizontal(|ui| {
                    ui.add_space(SPACING_MD);
                    
                    // 警告图标
                    egui::Frame::none()
                        .fill(Color32::from_rgba_unmultiplied(235, 87, 87, 25))
                        .rounding(Rounding::same(20.0))
                        .inner_margin(egui::Margin::same(8.0))
                        .show(ui, |ui| {
                            ui.label(RichText::new("⚠").size(20.0).color(DANGER));
                        });
                    
                    ui.add_space(SPACING_MD);
                    
                    // 消息文本
                    ui.vertical(|ui| {
                        ui.add_space(4.0);
                        ui.label(RichText::new(message).size(14.0));
                    });
                });

                ui.add_space(SPACING_LG);

                // 按钮区域
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // 确认按钮（危险样式）
                        let confirm_btn = egui::Button::new(
                            RichText::new(format!("{} [Enter]", confirm_text))
                                .color(Color32::WHITE)
                        )
                        .fill(DANGER)
                        .rounding(Rounding::same(6.0));

                        if ui.add(confirm_btn).clicked() {
                            *on_confirm = true;
                            *show = false;
                        }

                        ui.add_space(SPACING_MD);

                        // 取消按钮
                        let cancel_btn = egui::Button::new("取消 [Esc]")
                            .rounding(Rounding::same(6.0));

                        if ui.add(cancel_btn).clicked() {
                            *show = false;
                        }
                    });
                });

                ui.add_space(SPACING_MD);
            });
    }
}
