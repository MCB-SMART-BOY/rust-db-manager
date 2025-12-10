//! 筛选逻辑关系
//!
//! 定义筛选条件之间的 AND/OR 逻辑关系。

/// 筛选条件之间的逻辑关系
#[derive(Clone, Debug, Default, PartialEq, Copy)]
pub enum FilterLogic {
    #[default]
    And,
    Or,
}

impl FilterLogic {
    /// 获取显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::And => "AND",
            Self::Or => "OR",
        }
    }

    /// 切换逻辑关系
    pub fn toggle(&mut self) {
        *self = match self {
            Self::And => Self::Or,
            Self::Or => Self::And,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_and() {
        let logic = FilterLogic::default();
        assert_eq!(logic, FilterLogic::And);
    }

    #[test]
    fn test_toggle() {
        let mut logic = FilterLogic::And;
        logic.toggle();
        assert_eq!(logic, FilterLogic::Or);
        logic.toggle();
        assert_eq!(logic, FilterLogic::And);
    }

    #[test]
    fn test_display_name() {
        assert_eq!(FilterLogic::And.display_name(), "AND");
        assert_eq!(FilterLogic::Or.display_name(), "OR");
    }
}
