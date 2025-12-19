//! 对话框键盘导航模块
//!
//! 提供统一的 Helix 风格键盘导航支持，包括：
//! - 列表导航 (j/k)
//! - 选项切换 (h/l)
//! - 对话框控制 (Enter/Esc)
//! - 快速选择 (数字键)

#![allow(dead_code)] // 预留的辅助函数供未来使用

use egui::{Context, Key};

/// 对话框操作结果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DialogAction {
    /// 无操作
    #[default]
    None,
    /// 确认/提交
    Confirm,
    /// 取消/关闭
    Cancel,
}

/// 列表导航结果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListNavigation {
    /// 无导航
    #[default]
    None,
    /// 向上移动
    Up,
    /// 向下移动
    Down,
    /// 跳到开头
    Start,
    /// 跳到结尾
    End,
    /// 切换选中状态
    Toggle,
    /// 删除当前项
    Delete,
    /// 在下方添加
    AddBelow,
    /// 在上方添加
    AddAbove,
}

/// 水平导航结果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HorizontalNavigation {
    /// 无导航
    #[default]
    None,
    /// 向左移动
    Left,
    /// 向右移动
    Right,
}

/// 处理对话框基本快捷键 (Enter 确认, Esc 取消)
pub fn handle_dialog_keys(ctx: &Context) -> DialogAction {
    ctx.input(|i| {
        if i.key_pressed(Key::Escape) {
            return DialogAction::Cancel;
        }
        // Enter 确认（但不在有修饰键时触发，避免和其他快捷键冲突）
        if i.key_pressed(Key::Enter) && !i.modifiers.ctrl && !i.modifiers.shift && !i.modifiers.alt
        {
            return DialogAction::Confirm;
        }
        DialogAction::None
    })
}

/// 处理确认对话框快捷键 (y/n 风格)
pub fn handle_confirm_keys(ctx: &Context) -> DialogAction {
    ctx.input(|i| {
        if i.key_pressed(Key::Escape) || i.key_pressed(Key::N) {
            return DialogAction::Cancel;
        }
        if i.key_pressed(Key::Enter) || i.key_pressed(Key::Y) {
            return DialogAction::Confirm;
        }
        DialogAction::None
    })
}

/// 处理列表导航快捷键 (j/k 上下, g/G 首尾, Space 切换)
pub fn handle_list_navigation(ctx: &Context) -> ListNavigation {
    ctx.input(|i| {
        // 向下: j 或 下箭头
        if i.key_pressed(Key::J) || i.key_pressed(Key::ArrowDown) {
            return ListNavigation::Down;
        }
        // 向上: k 或 上箭头
        if i.key_pressed(Key::K) || i.key_pressed(Key::ArrowUp) {
            return ListNavigation::Up;
        }
        // 跳到开头: g (非 Shift)
        if i.key_pressed(Key::G) && !i.modifiers.shift {
            return ListNavigation::Start;
        }
        // 跳到结尾: G (Shift+g)
        if i.key_pressed(Key::G) && i.modifiers.shift {
            return ListNavigation::End;
        }
        // 切换: Space
        if i.key_pressed(Key::Space) {
            return ListNavigation::Toggle;
        }
        // 删除: d 或 Delete
        if i.key_pressed(Key::D) || i.key_pressed(Key::Delete) {
            return ListNavigation::Delete;
        }
        // 在下方添加: o
        if i.key_pressed(Key::O) && !i.modifiers.shift {
            return ListNavigation::AddBelow;
        }
        // 在上方添加: O (Shift+o)
        if i.key_pressed(Key::O) && i.modifiers.shift {
            return ListNavigation::AddAbove;
        }
        ListNavigation::None
    })
}

/// 处理水平导航快捷键 (h/l 左右)
pub fn handle_horizontal_navigation(ctx: &Context) -> HorizontalNavigation {
    ctx.input(|i| {
        if i.key_pressed(Key::H) || i.key_pressed(Key::ArrowLeft) {
            return HorizontalNavigation::Left;
        }
        if i.key_pressed(Key::L) || i.key_pressed(Key::ArrowRight) {
            return HorizontalNavigation::Right;
        }
        HorizontalNavigation::None
    })
}

/// 处理数字键快速选择 (1-9)
/// 返回 Some(index) 表示按下了数字键，index 从 0 开始
pub fn handle_number_select(ctx: &Context) -> Option<usize> {
    ctx.input(|i| {
        if i.key_pressed(Key::Num1) {
            return Some(0);
        }
        if i.key_pressed(Key::Num2) {
            return Some(1);
        }
        if i.key_pressed(Key::Num3) {
            return Some(2);
        }
        if i.key_pressed(Key::Num4) {
            return Some(3);
        }
        if i.key_pressed(Key::Num5) {
            return Some(4);
        }
        if i.key_pressed(Key::Num6) {
            return Some(5);
        }
        if i.key_pressed(Key::Num7) {
            return Some(6);
        }
        if i.key_pressed(Key::Num8) {
            return Some(7);
        }
        if i.key_pressed(Key::Num9) {
            return Some(8);
        }
        None
    })
}

/// 处理滚动快捷键 (用于帮助对话框等长内容)
/// 返回滚动增量（正数向下，负数向上）
pub fn handle_scroll_keys(ctx: &Context) -> f32 {
    ctx.input(|i| {
        let mut delta = 0.0;

        // j 或下箭头: 向下滚动
        if i.key_pressed(Key::J) || i.key_pressed(Key::ArrowDown) {
            delta = 50.0;
        }
        // k 或上箭头: 向上滚动
        if i.key_pressed(Key::K) || i.key_pressed(Key::ArrowUp) {
            delta = -50.0;
        }
        // Ctrl+d: 向下翻页
        if i.modifiers.ctrl && i.key_pressed(Key::D) {
            delta = 300.0;
        }
        // Ctrl+u: 向上翻页
        if i.modifiers.ctrl && i.key_pressed(Key::U) {
            delta = -300.0;
        }
        // g: 跳到开头 (返回一个大的负值)
        if i.key_pressed(Key::G) && !i.modifiers.shift {
            delta = -10000.0;
        }
        // G: 跳到结尾 (返回一个大的正值)
        if i.key_pressed(Key::G) && i.modifiers.shift {
            delta = 10000.0;
        }

        delta
    })
}

/// 处理关闭快捷键 (q 或 Esc)
pub fn handle_close_keys(ctx: &Context) -> bool {
    ctx.input(|i| i.key_pressed(Key::Escape) || i.key_pressed(Key::Q))
}

/// 应用列表导航到索引
/// 返回更新后的索引
pub fn apply_list_navigation(nav: ListNavigation, current: usize, count: usize) -> usize {
    if count == 0 {
        return 0;
    }
    match nav {
        ListNavigation::Down => (current + 1).min(count - 1),
        ListNavigation::Up => current.saturating_sub(1),
        ListNavigation::Start => 0,
        ListNavigation::End => count - 1,
        _ => current,
    }
}

/// 应用水平导航到索引
/// 返回更新后的索引
pub fn apply_horizontal_navigation(nav: HorizontalNavigation, current: usize, count: usize) -> usize {
    if count == 0 {
        return 0;
    }
    match nav {
        HorizontalNavigation::Left => current.saturating_sub(1),
        HorizontalNavigation::Right => (current + 1).min(count - 1),
        HorizontalNavigation::None => current,
    }
}

/// 检查是否有任何文本输入焦点
/// 用于决定是否处理键盘快捷键（避免和文本输入冲突）
pub fn has_text_focus(ctx: &Context) -> bool {
    ctx.memory(|m| m.focused().is_some())
}
