//! 筛选条件和筛选逻辑

use super::state::DataGridState;
use crate::database::QueryResult;
use crate::ui::styles::GRAY;
use egui::{self, Color32, RichText, TextEdit};

/// 列筛选条件
#[derive(Clone, Default)]
pub struct ColumnFilter {
    pub column: String,
    pub operator: FilterOperator,
    pub value: String,
}

#[derive(Clone, Default, PartialEq)]
pub enum FilterOperator {
    #[default]
    Contains,
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    IsNull,
    IsNotNull,
}

impl FilterOperator {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Contains => "包含",
            Self::Equals => "等于",
            Self::NotEquals => "不等于",
            Self::GreaterThan => "大于",
            Self::LessThan => "小于",
            Self::IsNull => "为空",
            Self::IsNotNull => "不为空",
        }
    }

    /// 操作符符号（用于紧凑显示）
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Contains => "~",
            Self::Equals => "=",
            Self::NotEquals => "≠",
            Self::GreaterThan => ">",
            Self::LessThan => "<",
            Self::IsNull => "∅",
            Self::IsNotNull => "≠∅",
        }
    }

    pub fn all() -> &'static [FilterOperator] {
        &[
            Self::Contains,
            Self::Equals,
            Self::NotEquals,
            Self::GreaterThan,
            Self::LessThan,
            Self::IsNull,
            Self::IsNotNull,
        ]
    }
}

/// 显示筛选栏
pub fn show_filter_bar(ui: &mut egui::Ui, result: &QueryResult, state: &mut DataGridState) {
    if state.filters.is_empty() {
        return;
    }

    ui.horizontal(|ui| {
        ui.label(RichText::new("筛选:").small().color(GRAY));

        let mut filter_to_remove: Option<usize> = None;

        for (idx, filter) in state.filters.iter_mut().enumerate() {
            egui::Frame::none()
                .fill(Color32::from_rgb(40, 45, 55))
                .rounding(4.0)
                .inner_margin(egui::Margin::symmetric(6.0, 2.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.set_height(22.0);

                        // 列选择
                        egui::ComboBox::from_id_source(format!("fc_{}", idx))
                            .selected_text(RichText::new(&filter.column).small())
                            .width(80.0)
                            .show_ui(ui, |ui| {
                                for col in &result.columns {
                                    ui.selectable_value(&mut filter.column, col.clone(), col);
                                }
                            });

                        // 操作符选择
                        egui::ComboBox::from_id_source(format!("fo_{}", idx))
                            .selected_text(RichText::new(filter.operator.symbol()).small())
                            .width(50.0)
                            .show_ui(ui, |ui| {
                                for op in FilterOperator::all() {
                                    ui.selectable_value(
                                        &mut filter.operator,
                                        op.clone(),
                                        format!("{} {}", op.symbol(), op.display_name()),
                                    );
                                }
                            });

                        // 值输入
                        if filter.operator != FilterOperator::IsNull
                            && filter.operator != FilterOperator::IsNotNull
                        {
                            ui.add(
                                TextEdit::singleline(&mut filter.value)
                                    .desired_width(100.0)
                                    .font(egui::TextStyle::Small)
                                    .hint_text("值..."),
                            );
                        }

                        // 删除按钮
                        if ui.small_button("✕").on_hover_text("删除 [x]").clicked() {
                            filter_to_remove = Some(idx);
                        }
                    });
                });

            ui.add_space(4.0);
        }

        // 清空全部按钮
        if state.filters.len() > 1
            && ui
                .small_button("清空")
                .on_hover_text("清空所有筛选 [Esc]")
                .clicked()
        {
            state.filters.clear();
        }

        if let Some(idx) = filter_to_remove {
            state.filters.remove(idx);
        }
    });
}

/// 过滤行数据
pub fn filter_rows<'a>(
    result: &'a QueryResult,
    search_text: &str,
    search_column: &Option<String>,
    filters: &[ColumnFilter],
) -> Vec<(usize, &'a Vec<String>)> {
    let search_lower = search_text.to_lowercase();

    result
        .rows
        .iter()
        .enumerate()
        .filter(|(_, row)| {
            // 搜索条件
            let search_match = if search_text.is_empty() {
                true
            } else {
                match search_column {
                    Some(col_name) => result
                        .columns
                        .iter()
                        .position(|c| c == col_name)
                        .and_then(|col_idx| row.get(col_idx))
                        .map(|cell| cell.to_lowercase().contains(&search_lower))
                        .unwrap_or(false),
                    None => row
                        .iter()
                        .any(|cell| cell.to_lowercase().contains(&search_lower)),
                }
            };

            if !search_match {
                return false;
            }

            // 筛选条件
            for filter in filters {
                let col_idx = result.columns.iter().position(|c| c == &filter.column);
                if let Some(idx) = col_idx {
                    if let Some(cell) = row.get(idx) {
                        if !check_filter_condition(cell, filter) {
                            return false;
                        }
                    }
                }
            }

            true
        })
        .collect()
}

/// 检查筛选条件
fn check_filter_condition(cell: &str, filter: &ColumnFilter) -> bool {
    let cell_lower = cell.to_lowercase();
    let value_lower = filter.value.to_lowercase();

    match filter.operator {
        FilterOperator::Contains => cell_lower.contains(&value_lower),
        FilterOperator::Equals => cell_lower == value_lower,
        FilterOperator::NotEquals => cell_lower != value_lower,
        FilterOperator::GreaterThan => cell
            .parse::<f64>()
            .ok()
            .zip(filter.value.parse::<f64>().ok())
            .map(|(a, b)| a > b)
            .unwrap_or_else(|| cell > filter.value.as_str()),
        FilterOperator::LessThan => cell
            .parse::<f64>()
            .ok()
            .zip(filter.value.parse::<f64>().ok())
            .map(|(a, b)| a < b)
            .unwrap_or_else(|| cell < filter.value.as_str()),
        FilterOperator::IsNull => cell == "NULL" || cell.is_empty(),
        FilterOperator::IsNotNull => cell != "NULL" && !cell.is_empty(),
    }
}
