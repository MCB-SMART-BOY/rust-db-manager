//! 数据库连接对话框

use crate::database::{ConnectionConfig, DatabaseType};
use crate::ui::styles::{DANGER, GRAY, MUTED, SUCCESS, SPACING_SM, SPACING_MD, SPACING_LG};
use egui::{self, Color32, RichText, Rounding, TextEdit};

pub struct ConnectionDialog;

impl ConnectionDialog {
    pub fn show(
        ctx: &egui::Context,
        open: &mut bool,
        config: &mut ConnectionConfig,
        on_save: &mut bool,
    ) {
        let mut is_open = *open;
        let mut should_close = false;

        egui::Window::new("🔗 新建数据库连接")
            .open(&mut is_open)
            .resizable(false)
            .collapsible(false)
            .min_width(480.0)
            .show(ctx, |ui| {
                ui.add_space(SPACING_MD);

                // 数据库类型选择卡片
                Self::show_db_type_selector(ui, config);

                ui.add_space(SPACING_LG);

                // 连接表单
                Self::show_connection_form(ui, config);

                ui.add_space(SPACING_LG);

                // 连接字符串预览
                Self::show_connection_preview(ui, config);

                ui.add_space(SPACING_LG);
                ui.separator();
                ui.add_space(SPACING_MD);

                // 底部按钮
                Self::show_buttons(ui, config, on_save, &mut should_close);

                ui.add_space(SPACING_SM);
            });

        if should_close {
            is_open = false;
        }
        *open = is_open;
    }

    /// 数据库类型选择器
    fn show_db_type_selector(ui: &mut egui::Ui, config: &mut ConnectionConfig) {
        ui.horizontal(|ui| {
            ui.add_space(SPACING_SM);
            
            for db_type in DatabaseType::all() {
                let is_selected = config.db_type == *db_type;
                let (icon, name, color) = match db_type {
                    DatabaseType::SQLite => ("🗃️", "SQLite", Color32::from_rgb(80, 160, 220)),
                    DatabaseType::PostgreSQL => ("🐘", "PostgreSQL", Color32::from_rgb(80, 130, 180)),
                    DatabaseType::MySQL => ("🐬", "MySQL", Color32::from_rgb(240, 150, 80)),
                };

                let fill = if is_selected {
                    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 40)
                } else {
                    Color32::TRANSPARENT
                };

                let stroke = if is_selected {
                    egui::Stroke::new(2.0, color)
                } else {
                    egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(150, 150, 160, 50))
                };

                let response = egui::Frame::none()
                    .fill(fill)
                    .stroke(stroke)
                    .rounding(Rounding::same(8.0))
                    .inner_margin(egui::Margin::symmetric(16.0, 10.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(icon).size(18.0));
                            ui.add_space(4.0);
                            let text_color = if is_selected { color } else { GRAY };
                            ui.label(RichText::new(name).strong().color(text_color));
                        });
                    })
                    .response
                    .interact(egui::Sense::click());

                if response.clicked() {
                    config.db_type = db_type.clone();
                    config.port = db_type.default_port();
                    if config.host.is_empty() && !matches!(db_type, DatabaseType::SQLite) {
                        config.host = "localhost".to_string();
                    }
                }

                ui.add_space(SPACING_SM);
            }
        });
    }

    /// 连接表单
    fn show_connection_form(ui: &mut egui::Ui, config: &mut ConnectionConfig) {
        egui::Frame::none()
            .fill(Color32::from_rgba_unmultiplied(100, 100, 110, 10))
            .rounding(Rounding::same(8.0))
            .inner_margin(egui::Margin::symmetric(16.0, 12.0))
            .show(ui, |ui| {
                egui::Grid::new("connection_form")
                    .num_columns(2)
                    .spacing([16.0, 10.0])
                    .show(ui, |ui| {
                        // 连接名称
                        ui.label(RichText::new("连接名称").color(GRAY));
                        ui.add(
                            TextEdit::singleline(&mut config.name)
                                .hint_text("我的数据库")
                                .desired_width(280.0)
                        );
                        ui.end_row();

                        if !matches!(config.db_type, DatabaseType::SQLite) {
                            // 主机地址
                            ui.label(RichText::new("主机地址").color(GRAY));
                            ui.add(
                                TextEdit::singleline(&mut config.host)
                                    .hint_text("localhost")
                                    .desired_width(280.0)
                            );
                            ui.end_row();

                            // 端口
                            ui.label(RichText::new("端口").color(GRAY));
                            let mut port_string = config.port.to_string();
                            ui.add(
                                TextEdit::singleline(&mut port_string)
                                    .char_limit(5)
                                    .desired_width(80.0)
                            );
                            if let Ok(port) = port_string.parse::<u16>() {
                                config.port = port;
                            }
                            ui.end_row();

                            // 用户名
                            ui.label(RichText::new("用户名").color(GRAY));
                            ui.add(
                                TextEdit::singleline(&mut config.username)
                                    .hint_text("root")
                                    .desired_width(280.0)
                            );
                            ui.end_row();

                            // 密码
                            ui.label(RichText::new("密码").color(GRAY));
                            ui.add(
                                TextEdit::singleline(&mut config.password)
                                    .password(true)
                                    .desired_width(280.0)
                            );
                            ui.end_row();
                        }

                        // 数据库/文件路径
                        let label = match config.db_type {
                            DatabaseType::SQLite => "文件路径",
                            _ => "数据库名",
                        };
                        ui.label(RichText::new(label).color(GRAY));

                        ui.horizontal(|ui| {
                            let hint = match config.db_type {
                                DatabaseType::SQLite => "/path/to/database.db",
                                _ => "database_name",
                            };
                            let width = if matches!(config.db_type, DatabaseType::SQLite) {
                                200.0
                            } else {
                                280.0
                            };
                            ui.add(
                                TextEdit::singleline(&mut config.database)
                                    .hint_text(hint)
                                    .desired_width(width)
                            );

                            if matches!(config.db_type, DatabaseType::SQLite) {
                                if ui.add(
                                    egui::Button::new("📂 浏览")
                                        .rounding(Rounding::same(4.0))
                                ).clicked() {
                                    if let Some(path) = rfd::FileDialog::new()
                                        .add_filter("SQLite 数据库", &["db", "sqlite", "sqlite3"])
                                        .add_filter("所有文件", &["*"])
                                        .pick_file()
                                    {
                                        config.database = path.display().to_string();
                                    }
                                }
                            }
                        });
                        ui.end_row();
                    });
            });

        // 提示信息
        ui.add_space(SPACING_SM);
        ui.horizontal(|ui| {
            ui.add_space(SPACING_MD);
            ui.label(RichText::new("💡").size(12.0));
            ui.add_space(4.0);
            let tip = match config.db_type {
                DatabaseType::SQLite => "输入 SQLite 数据库文件路径，文件不存在时将自动创建",
                DatabaseType::PostgreSQL => "默认端口 5432，请确保 PostgreSQL 服务已启动",
                DatabaseType::MySQL => "默认端口 3306，请确保 MySQL 服务已启动",
            };
            ui.label(RichText::new(tip).small().color(MUTED));
        });
    }

    /// 连接字符串预览
    fn show_connection_preview(ui: &mut egui::Ui, config: &ConnectionConfig) {
        ui.collapsing("🔍 连接字符串预览", |ui| {
            ui.add_space(SPACING_SM);
            
            egui::Frame::none()
                .fill(Color32::from_rgba_unmultiplied(60, 60, 70, 40))
                .rounding(Rounding::same(4.0))
                .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                .show(ui, |ui| {
                    let conn_str = config.connection_string();
                    let display_str = if !config.password.is_empty() {
                        conn_str.replace(&config.password, "****")
                    } else {
                        conn_str
                    };
                    ui.label(RichText::new(&display_str).monospace().small());
                });
        });
    }

    /// 底部按钮
    fn show_buttons(
        ui: &mut egui::Ui,
        config: &ConnectionConfig,
        on_save: &mut bool,
        should_close: &mut bool,
    ) {
        ui.horizontal(|ui| {
            // 取消按钮
            if ui.add(
                egui::Button::new("取消 [Esc]")
                    .rounding(Rounding::same(6.0))
            ).clicked() {
                *should_close = true;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let can_save = !config.name.is_empty()
                    && (matches!(config.db_type, DatabaseType::SQLite)
                        || (!config.host.is_empty() && !config.database.is_empty()));

                // 保存按钮
                let save_btn = egui::Button::new(
                    RichText::new("✓ 保存并连接 [Enter]")
                        .color(if can_save { Color32::WHITE } else { GRAY })
                )
                .fill(if can_save { SUCCESS } else { Color32::from_rgb(80, 80, 90) })
                .rounding(Rounding::same(6.0));

                if ui.add_enabled(can_save, save_btn).clicked() {
                    *on_save = true;
                    *should_close = true;
                }

                if !can_save {
                    ui.add_space(SPACING_MD);
                    ui.label(RichText::new("请填写必填项").small().color(DANGER));
                }
            });
        });
    }
}
