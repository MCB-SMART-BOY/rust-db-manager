//! 表格操作和 SQL 生成

use super::state::DataGridState;
use crate::database::QueryResult;

/// 表格操作返回值
#[derive(Default)]
pub struct DataGridActions {
    /// 需要执行的 SQL 语句列表
    pub sql_to_execute: Vec<String>,
    /// 状态消息
    pub message: Option<String>,
}

/// 验证并转义 SQL 标识符（表名、列名）
///
/// 防止 SQL 注入攻击，只允许合法的标识符字符
fn escape_identifier(name: &str) -> Result<String, String> {
    // 验证标识符：只允许字母、数字、下划线，且不能以数字开头
    if name.is_empty() {
        return Err("标识符不能为空".to_string());
    }

    let first_char = name.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return Err(format!("标识符 '{}' 不能以数字或特殊字符开头", name));
    }

    for c in name.chars() {
        if !c.is_alphanumeric() && c != '_' {
            return Err(format!("标识符 '{}' 包含非法字符 '{}'", name, c));
        }
    }

    // 使用双引号包裹（ANSI SQL 标准），适用于 PostgreSQL 和 SQLite
    // MySQL 使用反引号，但双引号在 ANSI_QUOTES 模式下也可用
    Ok(format!("\"{}\"", name))
}

/// 转义 SQL 字符串值
///
/// 处理单引号转义，防止 SQL 注入
fn escape_value(value: &str) -> String {
    if value == "NULL" {
        return "NULL".to_string();
    }
    // 转义单引号为两个单引号
    format!("'{}'", value.replace('\'', "''"))
}

/// 生成保存修改的 SQL（带确认）
pub fn generate_save_sql(
    result: &QueryResult,
    state: &mut DataGridState,
    table_name: &str,
    actions: &mut DataGridActions,
) {
    // 验证表名
    let safe_table_name = match escape_identifier(table_name) {
        Ok(name) => name,
        Err(e) => {
            actions.message = Some(format!("表名无效: {}", e));
            return;
        }
    };

    // 验证所有列名
    let mut safe_columns: Vec<String> = Vec::new();
    for col in &result.columns {
        match escape_identifier(col) {
            Ok(name) => safe_columns.push(name),
            Err(e) => {
                actions.message = Some(format!("列名无效: {}", e));
                return;
            }
        }
    }

    let mut sql_statements = Vec::new();
    let has_deletes = !state.rows_to_delete.is_empty();

    // 获取主键列（第一列）
    let pk_col = safe_columns.first().cloned().unwrap_or_else(|| "\"id\"".to_string());

    // 生成 UPDATE 语句
    for ((row_idx, col_idx), new_value) in &state.modified_cells {
        if let Some(row) = result.rows.get(*row_idx) {
            if let Some(pk_value) = row.first() {
                if let Some(col_name) = safe_columns.get(*col_idx) {
                    let safe_value = if new_value.is_empty() || new_value.eq_ignore_ascii_case("null") {
                        "NULL".to_string()
                    } else {
                        escape_value(new_value)
                    };
                    let safe_pk_value = escape_value(pk_value);

                    let sql = format!(
                        "UPDATE {} SET {} = {} WHERE {} = {};",
                        safe_table_name, col_name, safe_value, pk_col, safe_pk_value
                    );
                    sql_statements.push(sql);
                }
            }
        }
    }

    // 生成 DELETE 语句
    for row_idx in &state.rows_to_delete {
        if let Some(row) = result.rows.get(*row_idx) {
            if let Some(pk_value) = row.first() {
                let safe_pk_value = escape_value(pk_value);
                let sql = format!(
                    "DELETE FROM {} WHERE {} = {};",
                    safe_table_name, pk_col, safe_pk_value
                );
                sql_statements.push(sql);
            }
        }
    }

    // 生成 INSERT 语句
    for new_row in &state.new_rows {
        if new_row.iter().any(|v| !v.is_empty()) {
            let cols = safe_columns.join(", ");
            let vals: Vec<String> = new_row
                .iter()
                .map(|v| {
                    if v.is_empty() || v.eq_ignore_ascii_case("null") {
                        "NULL".to_string()
                    } else {
                        escape_value(v)
                    }
                })
                .collect();
            let sql = format!(
                "INSERT INTO {} ({}) VALUES ({});",
                safe_table_name,
                cols,
                vals.join(", ")
            );
            sql_statements.push(sql);
        }
    }

    if sql_statements.is_empty() {
        actions.message = Some("没有需要保存的修改".to_string());
        return;
    }

    // 如果包含删除操作，需要确认
    if has_deletes {
        state.pending_sql = sql_statements;
        state.show_save_confirm = true;
        actions.message = Some(format!(
            "包含 {} 条删除操作，请确认",
            state.rows_to_delete.len()
        ));
    } else {
        // 没有删除操作，直接执行
        actions.sql_to_execute = sql_statements;
        actions.message = Some(format!(
            "将执行 {} 条 SQL 语句",
            actions.sql_to_execute.len()
        ));
        state.clear_edits();
    }
}

/// 确认执行待确认的 SQL
pub fn confirm_pending_sql(state: &mut DataGridState, actions: &mut DataGridActions) {
    if !state.pending_sql.is_empty() {
        actions.sql_to_execute = std::mem::take(&mut state.pending_sql);
        actions.message = Some(format!("执行 {} 条 SQL 语句", actions.sql_to_execute.len()));
        state.clear_edits();
    }
    state.show_save_confirm = false;
}

/// 取消待确认的 SQL
pub fn cancel_pending_sql(state: &mut DataGridState) {
    state.pending_sql.clear();
    state.show_save_confirm = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_identifier_valid() {
        assert_eq!(escape_identifier("users").unwrap(), "\"users\"");
        assert_eq!(escape_identifier("user_name").unwrap(), "\"user_name\"");
        assert_eq!(escape_identifier("_private").unwrap(), "\"_private\"");
        assert_eq!(escape_identifier("Table123").unwrap(), "\"Table123\"");
    }

    #[test]
    fn test_escape_identifier_invalid() {
        assert!(escape_identifier("").is_err());
        assert!(escape_identifier("123table").is_err());
        assert!(escape_identifier("user-name").is_err());
        assert!(escape_identifier("user;drop").is_err());
        assert!(escape_identifier("table name").is_err());
    }

    #[test]
    fn test_escape_value() {
        assert_eq!(escape_value("hello"), "'hello'");
        assert_eq!(escape_value("it's"), "'it''s'");
        assert_eq!(escape_value("NULL"), "NULL");
        assert_eq!(escape_value("O'Brien"), "'O''Brien'");
    }
}
