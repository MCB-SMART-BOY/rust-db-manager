#![allow(clippy::too_many_arguments)]

use crate::core::{ThemeManager, ThemePreset};
use crate::ui::styles::{MUTED, SPACING_MD, SPACING_SM};
use egui::{self, Color32, Id, Key, RichText, Rounding, Vec2};

/// Helix 风格下拉框状态
#[derive(Default, Clone)]
struct ComboState {
    selected_index: usize,
    is_open: bool,
}

/// 主题下拉框状态（带分组）
#[derive(Default, Clone)]
struct ThemeComboState {
    selected_index: usize,
    is_open: bool,
}

pub struct Toolbar;

#[derive(Default)]
pub struct ToolbarActions {
    pub refresh_tables: bool,
    pub export: bool,
    pub import: bool,
    pub show_history: bool,
    pub show_help: bool,
    pub toggle_sidebar: bool,
    pub toggle_editor: bool,
    pub show_editor: bool,
    pub theme_changed: Option<ThemePreset>,
    pub toggle_dark_mode: bool,
    pub switch_connection: Option<String>,
    pub switch_table: Option<String>,
    // 快捷键触发的下拉框打开
    pub open_theme_selector: bool,
    pub open_connection_selector: bool,
    pub open_table_selector: bool,
}

// 暗色主题列表
const DARK_THEMES: &[ThemePreset] = &[
    ThemePreset::TokyoNight,
    ThemePreset::TokyoNightStorm,
    ThemePreset::CatppuccinMocha,
    ThemePreset::CatppuccinMacchiato,
    ThemePreset::CatppuccinFrappe,
    ThemePreset::OneDark,
    ThemePreset::OneDarkVivid,
    ThemePreset::GruvboxDark,
    ThemePreset::Dracula,
    ThemePreset::Nord,
    ThemePreset::SolarizedDark,
    ThemePreset::MonokaiPro,
    ThemePreset::GithubDark,
];

// 亮色主题列表
const LIGHT_THEMES: &[ThemePreset] = &[
    ThemePreset::TokyoNightLight,
    ThemePreset::CatppuccinLatte,
    ThemePreset::OneLight,
    ThemePreset::GruvboxLight,
    ThemePreset::SolarizedLight,
    ThemePreset::GithubLight,
];

impl Toolbar {
    pub fn show(
        ui: &mut egui::Ui,
        theme_manager: &ThemeManager,
        has_result: bool,
        show_sidebar: bool,
        show_editor: bool,
        is_dark_mode: bool,
        actions: &mut ToolbarActions,
        connections: &[String],
        active_connection: Option<&str>,
        tables: &[String],
        selected_table: Option<&str>,
    ) {
        actions.show_editor = show_editor;

        // 工具栏容器
        egui::Frame::none()
            .inner_margin(egui::Margin::symmetric(SPACING_MD, SPACING_SM))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(6.0, 0.0);

                    // 左侧按钮组
                    Self::show_left_buttons(ui, show_sidebar, show_editor, actions);

                    ui.add_space(4.0);
                    Self::separator(ui);
                    ui.add_space(4.0);

                    // 连接和表选择器
                    Self::show_selectors(
                        ui,
                        connections,
                        active_connection,
                        tables,
                        selected_table,
                        actions,
                    );

                    ui.add_space(4.0);
                    Self::separator(ui);
                    ui.add_space(4.0);

                    // 操作按钮
                    Self::show_action_buttons(ui, has_result, actions);

                    // 右侧区域
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // 版本号
                        ui.label(RichText::new("v0.1.0").small().color(MUTED));
                        ui.add_space(8.0);
                        Self::separator(ui);
                        ui.add_space(8.0);

                        // 主题选择器 - 根据当前模式显示对应主题列表
                        let themes = if is_dark_mode { DARK_THEMES } else { LIGHT_THEMES };
                        let current_theme_idx = themes
                            .iter()
                            .position(|&t| t == theme_manager.current)
                            .unwrap_or(0);

                        if let Some(new_idx) = Self::helix_theme_combo_simple(
                            ui,
                            "theme_selector",
                            theme_manager.current,
                            current_theme_idx,
                            themes,
                            200.0,
                            actions.open_theme_selector,
                        ) {
                            if let Some(&preset) = themes.get(new_idx) {
                                actions.theme_changed = Some(preset);
                            }
                        }
                        actions.open_theme_selector = false;

                        ui.add_space(4.0);

                        // 日/夜模式切换按钮
                        let mode_icon = if is_dark_mode { "🌙" } else { "☀" };
                        let mode_tooltip = if is_dark_mode {
                            "切换到日间模式 [Ctrl+D]"
                        } else {
                            "切换到夜间模式 [Ctrl+D]"
                        };

                        let btn = Self::styled_button(mode_icon);
                        if ui.add(btn).on_hover_text(mode_tooltip).clicked() {
                            actions.toggle_dark_mode = true;
                        }
                    });
                });
            });
    }

    /// 显示左侧按钮
    fn show_left_buttons(
        ui: &mut egui::Ui,
        show_sidebar: bool,
        show_editor: bool,
        actions: &mut ToolbarActions,
    ) {
        // 侧边栏切换
        let sidebar_icon = if show_sidebar { "◀" } else { "▶" };
        if ui.add(Self::styled_button(&format!("{} 侧栏 [Ctrl+B]", sidebar_icon))).clicked() {
            actions.toggle_sidebar = true;
        }

        // 编辑器切换
        let editor_icon = if show_editor { "▼" } else { "▲" };
        if ui.add(Self::styled_button(&format!("{} 编辑器 [Ctrl+J]", editor_icon))).clicked() {
            actions.toggle_editor = true;
        }
    }

    /// 显示选择器
    fn show_selectors(
        ui: &mut egui::Ui,
        connections: &[String],
        active_connection: Option<&str>,
        tables: &[String],
        selected_table: Option<&str>,
        actions: &mut ToolbarActions,
    ) {
        // 连接选择器
        if let Some(idx) = Self::helix_combo(
            ui,
            "connection_selector",
            "🔌",
            "连接",
            connections,
            active_connection.and_then(|ac| connections.iter().position(|c| c == ac)),
            180.0,
            actions.open_connection_selector,
        ) {
            if let Some(conn_name) = connections.get(idx) {
                actions.switch_connection = Some(conn_name.clone());
            }
        }
        actions.open_connection_selector = false;

        // 表选择器
        if let Some(idx) = Self::helix_combo(
            ui,
            "table_selector",
            "📋",
            "表",
            tables,
            selected_table.and_then(|st| tables.iter().position(|t| t == st)),
            180.0,
            actions.open_table_selector,
        ) {
            if let Some(table_name) = tables.get(idx) {
                actions.switch_table = Some(table_name.clone());
            }
        }
        actions.open_table_selector = false;
    }

    /// 显示操作按钮
    fn show_action_buttons(ui: &mut egui::Ui, has_result: bool, actions: &mut ToolbarActions) {
        // 刷新
        if ui.add(Self::styled_button("🔄 刷新 [F5]")).clicked() {
            actions.refresh_tables = true;
        }

        // 导出
        if ui
            .add_enabled(has_result, Self::styled_button("📤 导出 [Ctrl+E]"))
            .clicked()
        {
            actions.export = true;
        }

        // 导入
        if ui.add(Self::styled_button("📥 导入 [Ctrl+I]")).clicked() {
            actions.import = true;
        }

        Self::separator(ui);

        // 历史
        if ui.add(Self::styled_button("📜 历史 [Ctrl+H]")).clicked() {
            actions.show_history = true;
        }

        // 帮助
        if ui.add(Self::styled_button("❓ 帮助 [F1]")).clicked() {
            actions.show_help = true;
        }
    }

    /// 样式化按钮
    fn styled_button(text: &str) -> egui::Button<'_> {
        egui::Button::new(RichText::new(text).size(13.0))
            .rounding(Rounding::same(6.0))
            .min_size(Vec2::new(0.0, 28.0))
    }

    /// 分隔符
    fn separator(ui: &mut egui::Ui) {
        ui.add_space(2.0);
        let rect = ui.available_rect_before_wrap();
        let height = 20.0;
        let y_center = rect.center().y;
        ui.painter().vline(
            rect.left(),
            (y_center - height / 2.0)..=(y_center + height / 2.0),
            egui::Stroke::new(1.0, Color32::from_white_alpha(30)),
        );
        ui.add_space(2.0);
    }

    /// Helix 风格下拉选择框
    fn helix_combo(
        ui: &mut egui::Ui,
        id_source: &str,
        icon: &str,
        label: &str,
        items: &[String],
        selected_index: Option<usize>,
        width: f32,
        force_open: bool,
    ) -> Option<usize> {
        let id = Id::new(id_source);
        let popup_id = id.with("popup");
        let mut result = None;

        // 获取状态
        let mut state = ui
            .ctx()
            .data_mut(|d| d.get_temp::<ComboState>(id).unwrap_or_default());

        // 快捷键触发打开
        if force_open && !items.is_empty() {
            state.is_open = true;
        }

        // 同步选中索引
        if let Some(idx) = selected_index {
            if !state.is_open {
                state.selected_index = idx;
            }
        }

        // 显示文本
        let display_text = if items.is_empty() {
            format!("{} {}: --", icon, label)
        } else {
            let item_text = selected_index
                .and_then(|i| items.get(i))
                .map(|s| truncate_text(s, 15))
                .unwrap_or_else(|| "选择...".to_string());
            format!("{} {}: {} ▾", icon, label, item_text)
        };

        // 按钮
        let response = ui
            .add(Self::styled_button(&display_text))
            .on_hover_text(format!("选择{} [j/k 导航]", label));

        if response.clicked() && !items.is_empty() {
            state.is_open = !state.is_open;
        }

        // 弹出菜单
        if state.is_open && !items.is_empty() {
            egui::Area::new(popup_id)
                .order(egui::Order::Foreground)
                .fixed_pos(response.rect.left_bottom() + Vec2::new(0.0, 4.0))
                .show(ui.ctx(), |ui| {
                    egui::Frame::popup(ui.style())
                        .rounding(Rounding::same(8.0))
                        .shadow(egui::epaint::Shadow {
                            offset: Vec2::new(0.0, 4.0),
                            blur: 12.0,
                            spread: 0.0,
                            color: Color32::from_black_alpha(60),
                        })
                        .show(ui, |ui| {
                            ui.set_min_width(width.min(200.0));
                            ui.set_max_width(width);

                            // 键盘处理
                            let input_result = ui.input(|i| {
                                let mut close = false;
                                let mut confirm = false;
                                let mut new_idx: Option<usize> = None;

                                if i.key_pressed(Key::J) || i.key_pressed(Key::ArrowDown) {
                                    let next = state.selected_index.saturating_add(1);
                                    if next < items.len() {
                                        new_idx = Some(next);
                                    }
                                }

                                if (i.key_pressed(Key::K) || i.key_pressed(Key::ArrowUp))
                                    && state.selected_index > 0
                                {
                                    new_idx = Some(state.selected_index - 1);
                                }

                                if i.key_pressed(Key::Enter) || i.key_pressed(Key::L) {
                                    confirm = true;
                                    close = true;
                                }

                                if i.key_pressed(Key::Escape) || i.key_pressed(Key::H) {
                                    close = true;
                                }

                                if i.key_pressed(Key::G) && !i.modifiers.shift {
                                    new_idx = Some(0);
                                }

                                if i.key_pressed(Key::G) && i.modifiers.shift {
                                    new_idx = Some(items.len().saturating_sub(1));
                                }

                                (close, confirm, new_idx)
                            });

                            if let Some(new_idx) = input_result.2 {
                                state.selected_index = new_idx;
                            }

                            if input_result.0 {
                                if input_result.1 {
                                    result = Some(state.selected_index);
                                }
                                state.is_open = false;
                            }

                            // 选项列表
                            egui::ScrollArea::vertical()
                                .max_height(280.0)
                                .show(ui, |ui| {
                                    ui.add_space(4.0);
                                    for (idx, item) in items.iter().enumerate() {
                                        let is_hover = idx == state.selected_index;
                                        let item_response =
                                            Self::render_combo_item(ui, item, is_hover, false);

                                        // 键盘选中时自动滚动到该项
                                        if is_hover {
                                            item_response.scroll_to_me(Some(egui::Align::Center));
                                        }

                                        if item_response.clicked() {
                                            result = Some(idx);
                                            state.is_open = false;
                                        }

                                        if item_response.hovered() {
                                            state.selected_index = idx;
                                        }
                                    }
                                    ui.add_space(4.0);
                                });

                            // 提示
                            ui.separator();
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                ui.label(
                                    RichText::new("j/k 选择  Enter 确认  Esc 取消")
                                        .small()
                                        .color(MUTED),
                                );
                            });
                            ui.add_space(4.0);
                        });
                });

            // 点击外部关闭
            let click_outside = ui.input(|i| {
                i.pointer.any_click()
                    && !response
                        .rect
                        .contains(i.pointer.interact_pos().unwrap_or_default())
            });
            if click_outside {
                state.is_open = false;
            }
        }

        // 保存状态
        ui.ctx().data_mut(|d| {
            d.insert_temp(id, state);
        });

        result
    }

    /// 简化版主题选择下拉框（只显示指定的主题列表）
    fn helix_theme_combo_simple(
        ui: &mut egui::Ui,
        id_source: &str,
        current_theme: ThemePreset,
        selected_index: usize,
        themes: &[ThemePreset],
        width: f32,
        force_open: bool,
    ) -> Option<usize> {
        let id = Id::new(id_source);
        let popup_id = id.with("popup");
        let mut result = None;

        // 获取状态
        let mut state = ui
            .ctx()
            .data_mut(|d| d.get_temp::<ThemeComboState>(id).unwrap_or_default());

        // 快捷键触发打开
        if force_open {
            state.is_open = true;
        }

        // 同步选中索引
        if !state.is_open {
            state.selected_index = selected_index;
        }

        // 按钮
        let display_text = format!("🎨 {} ▾", current_theme.display_name());
        let response = ui
            .add(Self::styled_button(&display_text))
            .on_hover_text("选择主题");

        if response.clicked() {
            state.is_open = !state.is_open;
        }

        // 弹出菜单
        if state.is_open {
            egui::Area::new(popup_id)
                .order(egui::Order::Foreground)
                .fixed_pos(
                    response.rect.left_bottom()
                        - Vec2::new(width - response.rect.width(), -4.0),
                )
                .show(ui.ctx(), |ui| {
                    egui::Frame::popup(ui.style())
                        .rounding(Rounding::same(8.0))
                        .shadow(egui::epaint::Shadow {
                            offset: Vec2::new(0.0, 4.0),
                            blur: 12.0,
                            spread: 0.0,
                            color: Color32::from_black_alpha(60),
                        })
                        .show(ui, |ui| {
                            ui.set_min_width(width.min(200.0));
                            ui.set_max_width(width);

                            let themes_len = themes.len();

                            // 键盘处理
                            let input_result = ui.input(|i| {
                                let mut close = false;
                                let mut confirm = false;
                                let mut new_idx: Option<usize> = None;

                                if i.key_pressed(Key::J) || i.key_pressed(Key::ArrowDown) {
                                    let next = state.selected_index.saturating_add(1);
                                    if next < themes_len {
                                        new_idx = Some(next);
                                    }
                                }

                                if (i.key_pressed(Key::K) || i.key_pressed(Key::ArrowUp))
                                    && state.selected_index > 0
                                {
                                    new_idx = Some(state.selected_index - 1);
                                }

                                if i.key_pressed(Key::Enter) || i.key_pressed(Key::L) {
                                    confirm = true;
                                    close = true;
                                }

                                if i.key_pressed(Key::Escape) || i.key_pressed(Key::H) {
                                    close = true;
                                }

                                if i.key_pressed(Key::G) && !i.modifiers.shift {
                                    new_idx = Some(0);
                                }

                                if i.key_pressed(Key::G) && i.modifiers.shift {
                                    new_idx = Some(themes_len.saturating_sub(1));
                                }

                                (close, confirm, new_idx)
                            });

                            if let Some(new_idx) = input_result.2 {
                                state.selected_index = new_idx;
                            }

                            if input_result.0 {
                                if input_result.1 {
                                    result = Some(state.selected_index);
                                }
                                state.is_open = false;
                            }

                            // 主题列表
                            egui::ScrollArea::vertical()
                                .max_height(300.0)
                                .show(ui, |ui| {
                                    ui.add_space(4.0);
                                    for (idx, theme) in themes.iter().enumerate() {
                                        let is_hover = idx == state.selected_index;
                                        let item_response =
                                            Self::render_combo_item(ui, theme.display_name(), is_hover, false);

                                        // 键盘选中时自动滚动到该项
                                        if is_hover {
                                            item_response.scroll_to_me(Some(egui::Align::Center));
                                        }

                                        if item_response.clicked() {
                                            result = Some(idx);
                                            state.is_open = false;
                                        }

                                        if item_response.hovered() {
                                            state.selected_index = idx;
                                        }
                                    }
                                    ui.add_space(4.0);
                                });

                            // 提示
                            ui.separator();
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                ui.label(
                                    RichText::new("j/k 选择  Enter 确认  Esc 取消")
                                        .small()
                                        .color(MUTED),
                                );
                            });
                            ui.add_space(4.0);
                        });
                });

            // 点击外部关闭
            let click_outside = ui.input(|i| {
                i.pointer.any_click()
                    && !response
                        .rect
                        .contains(i.pointer.interact_pos().unwrap_or_default())
            });
            if click_outside {
                state.is_open = false;
            }
        }

        // 保存状态
        ui.ctx().data_mut(|d| {
            d.insert_temp(id, state);
        });

        result
    }

    /// 渲染下拉选项
    fn render_combo_item(
        ui: &mut egui::Ui,
        text: &str,
        is_hover: bool,
        is_light_theme: bool,
    ) -> egui::Response {
        let bg_color = if is_hover {
            Color32::from_rgba_unmultiplied(100, 140, 200, 40)
        } else {
            Color32::TRANSPARENT
        };

        let frame_response = egui::Frame::none()
            .fill(bg_color)
            .inner_margin(egui::Margin::symmetric(12.0, 6.0))
            .rounding(4.0)
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    // 选中指示器
                    let indicator = if is_hover { "▸" } else { " " };
                    ui.label(
                        RichText::new(indicator)
                            .color(Color32::from_rgb(130, 180, 255))
                            .monospace(),
                    );

                    // 主题名称
                    let text_color = if is_hover {
                        Color32::from_rgb(200, 220, 255)
                    } else {
                        Color32::from_rgb(180, 180, 190)
                    };
                    ui.label(RichText::new(text).color(text_color));

                    // 浅色主题标识
                    if is_light_theme {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new("☀").small().color(Color32::from_rgb(255, 200, 100)));
                        });
                    }
                });
            });

        frame_response.response.interact(egui::Sense::click())
    }
}

/// 截断文本
fn truncate_text(text: &str, max_len: usize) -> String {
    if text.chars().count() > max_len {
        format!("{}…", text.chars().take(max_len - 1).collect::<String>())
    } else {
        text.to_string()
    }
}
