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
//! - 19 种主题预设
//! - 查询历史记录
//!
//! ## 模块结构
//!
//! - `app`: 主应用程序逻辑
//! - `core`: 核心功能（配置、主题、导出等）
//! - `database`: 数据库连接和查询
//! - `ui`: 用户界面组件

mod app;
mod core;
mod database;
mod ui;

use app::DbManagerApp;
use eframe::egui;

/// 程序入口点
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Rust 数据库管理器"),
        ..Default::default()
    };

    eframe::run_native(
        "Rust 数据库管理器",
        options,
        Box::new(|cc| {
            // 配置中文字体
            setup_fonts(&cc.egui_ctx);
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // 根据屏幕 DPI 自动缩放界面
            // egui 默认会使用系统 DPI 设置，这里确保字体大小合适
            let pixels_per_point = cc.egui_ctx.pixels_per_point();
            if pixels_per_point > 1.0 {
                cc.egui_ctx.set_pixels_per_point(pixels_per_point);
            }

            Box::new(DbManagerApp::new(cc))
        }),
    )
}

/// 内嵌的中文字体（霞鹜文楷 Lite）
const EMBEDDED_CHINESE_FONT: &[u8] = include_bytes!("../assets/fonts/LXGWWenKaiLite-Regular.ttf");

/// 配置字体
///
/// 使用内嵌的中文字体，确保在所有平台上都能正确显示中文。
fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 使用内嵌的中文字体
    fonts.font_data.insert(
        "chinese_font".to_owned(),
        egui::FontData::from_static(EMBEDDED_CHINESE_FONT),
    );

    // 配置字体优先级：中文字体优先
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "chinese_font".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "chinese_font".to_owned());

    ctx.set_fonts(fonts);
}
