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

/// 配置字体
///
/// 加载中文字体和符号字体，确保 Unicode 符号正确显示。
fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 系统中文字体路径列表
    let cjk_font_paths = [
        // Arch Linux noto-fonts-cjk
        "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/noto-cjk/NotoSansCJK-Medium.ttc",
        // 其他 Linux 发行版
        "/usr/share/fonts/google-noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/OTF/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/TTF/NotoSansCJK-Regular.ttc",
        // 文泉驿字体
        "/usr/share/fonts/wenquanyi/wqy-microhei/wqy-microhei.ttc",
        "/usr/share/fonts/wenquanyi/wqy-zenhei/wqy-zenhei.ttc",
        // 思源字体
        "/usr/share/fonts/adobe-source-han-sans/SourceHanSansCN-Regular.otf",
        "/usr/share/fonts/TTF/SourceHanSansCN-Regular.otf",
    ];

    // 符号字体路径（支持 Unicode 符号如 ▸ ▾ 等）
    let symbol_font_paths = [
        "/usr/share/fonts/noto/NotoSansSymbols2-Regular.ttf",
        "/usr/share/fonts/TTF/NotoSansSymbols2-Regular.ttf",
        "/usr/share/fonts/truetype/noto/NotoSansSymbols2-Regular.ttf",
        "/usr/share/fonts/noto/NotoSansSymbols-Regular.ttf",
        "/usr/share/fonts/TTF/NotoSansSymbols-Regular.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/TTF/DejaVuSans.ttf",
    ];

    let mut found_cjk = false;
    let mut found_symbol = false;

    // 加载中文字体
    for path in &cjk_font_paths {
        if let Ok(font_data) = std::fs::read(path) {
            fonts.font_data.insert(
                "chinese_font".to_owned(),
                egui::FontData::from_owned(font_data),
            );
            found_cjk = true;
            break;
        }
    }

    // 加载符号字体
    for path in &symbol_font_paths {
        if let Ok(font_data) = std::fs::read(path) {
            fonts.font_data.insert(
                "symbol_font".to_owned(),
                egui::FontData::from_owned(font_data),
            );
            found_symbol = true;
            break;
        }
    }

    // 配置字体优先级：中文字体 -> 符号字体 -> 默认字体
    if found_cjk {
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
    }

    if found_symbol {
        // 符号字体放在中文字体之后，作为 fallback
        let pos = if found_cjk { 1 } else { 0 };

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(pos, "symbol_font".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(pos, "symbol_font".to_owned());
    }

    ctx.set_fonts(fonts);

    if !found_cjk {
        eprintln!("警告: 未找到中文字体，请安装 noto-fonts-cjk 字体包");
    }
}
