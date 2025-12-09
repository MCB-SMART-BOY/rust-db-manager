//! 数据库连接对话框

use crate::database::{ConnectionConfig, DatabaseType};
use crate::ui::styles::{DANGER, GRAY, MUTED, SUCCESS, SPACING_SM, SPACING_MD, SPACING_LG};
use egui::{self, Color32, RichText, Rounding, TextEdit};
use std::path::Path;

/// 输入验证结果
struct ValidationResult {
    is_valid: bool,
    errors: Vec<String>,
}

impl ValidationResult {
    fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
        }
    }

    fn add_error(&mut self, error: impl Into<String>) {
        self.is_valid = false;
        self.errors.push(error.into());
    }
}

/// 验证连接配置
fn validate_config(config: &ConnectionConfig) -> ValidationResult {
    let mut result = ValidationResult::new();

    // 验证连接名称
    if config.name.is_empty() {
        result.add_error("连接名称不能为空");
    } else if config.name.len() > 64 {
        result.add_error("连接名称不能超过 64 个字符");
    }

    match config.db_type {
        DatabaseType::SQLite => {
            // SQLite 验证
            if config.database.is_empty() {
                result.add_error("数据库文件路径不能为空");
            } else {
                let path = Path::new(&config.database);
                // 检查父目录是否存在
                if let Some(parent) = path.parent() {
                    if !parent.as_os_str().is_empty() && !parent.exists() {
                        result.add_error(format!("目录不存在: {}", parent.display()));
                    }
                }
                // 检查文件扩展名
                if let Some(ext) = path.extension() {
                    let ext_lower = ext.to_string_lossy().to_lowercase();
                    if !["db", "sqlite", "sqlite3", "s3db"].contains(&ext_lower.as_str()) {
                        // 只是警告，不阻止保存
                    }
                }
            }
        }
        DatabaseType::PostgreSQL | DatabaseType::MySQL => {
            // 主机验证
            if config.host.is_empty() {
                result.add_error("主机地址不能为空");
            } else if config.host.contains(' ') {
                result.add_error("主机地址不能包含空格");
            } else if config.host.len() > 255 {
                result.add_error("主机地址过长");
            }

            // 端口验证（u16 类型范围已确保 0-65535）
            if config.port == 0 {
                result.add_error("端口号不能为 0");
            }
            // 注: 小于 1024 的端口是系统保留端口，但某些数据库可能使用

            // 用户名验证（可选但推荐）
            if config.username.len() > 128 {
                result.add_error("用户名过长");
            }
        }
    }

    result
}

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
                                .char_limit(64)
                                .desired_width(280.0)
                        );
                        ui.end_row();

                        if !matches!(config.db_type, DatabaseType::SQLite) {
                            // 主机地址
                            ui.label(RichText::new("主机地址").color(GRAY));
                            ui.add(
                                TextEdit::singleline(&mut config.host)
                                    .hint_text("localhost")
                                    .char_limit(255)
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
                                    .char_limit(128)
                                    .desired_width(280.0)
                            );
                            ui.end_row();

                            // 密码
                            ui.label(RichText::new("密码").color(GRAY));
                            ui.add(
                                TextEdit::singleline(&mut config.password)
                                    .password(true)
                                    .char_limit(256)
                                    .desired_width(280.0)
                            );
                            ui.end_row();
                        }

                        // SQLite 文件路径（必填）
                        if matches!(config.db_type, DatabaseType::SQLite) {
                            ui.label(RichText::new("文件路径").color(GRAY));

                            ui.horizontal(|ui| {
                                ui.add(
                                    TextEdit::singleline(&mut config.database)
                                        .hint_text("/path/to/database.db")
                                        .desired_width(200.0)
                                );

                                if ui.add(
                                    egui::Button::new("浏览")
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
                            });
                            ui.end_row();
                        }
                    });
            });

        // 提示信息
        ui.add_space(SPACING_SM);
        ui.horizontal(|ui| {
            ui.add_space(SPACING_MD);
            ui.add_space(4.0);
            let tip = match config.db_type {
                DatabaseType::SQLite => "输入 SQLite 数据库文件路径，文件不存在时将自动创建",
                DatabaseType::PostgreSQL => "默认端口 5432，连接后可选择数据库",
                DatabaseType::MySQL => "默认端口 3306，连接后可选择数据库",
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
        // 执行验证
        let validation = validate_config(config);

        ui.horizontal(|ui| {
            // 取消按钮
            if ui.add(
                egui::Button::new("取消 [Esc]")
                    .rounding(Rounding::same(6.0))
            ).clicked() {
                *should_close = true;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // 保存按钮
                let save_btn = egui::Button::new(
                    RichText::new("✓ 保存并连接 [Enter]")
                        .color(if validation.is_valid { Color32::WHITE } else { GRAY })
                )
                .fill(if validation.is_valid { SUCCESS } else { Color32::from_rgb(80, 80, 90) })
                .rounding(Rounding::same(6.0));

                if ui.add_enabled(validation.is_valid, save_btn).clicked() {
                    *on_save = true;
                    *should_close = true;
                }

                // 显示验证错误
                if !validation.is_valid {
                    ui.add_space(SPACING_MD);
                    // 只显示第一个错误
                    if let Some(error) = validation.errors.first() {
                        ui.label(RichText::new(error).small().color(DANGER));
                    }
                }
            });
        });
    }
}
