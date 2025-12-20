//! 数据库错误类型

use super::DatabaseType;
use thiserror::Error;

/// 数据库操作错误
#[allow(dead_code)] // 公开 API，供外部使用
#[derive(Error, Debug)]
pub enum DbError {
    #[error("连接错误: {0}")]
    Connection(String),
    #[error("连接错误 [{db_type}]: {message}")]
    ConnectionTyped { db_type: String, message: String },
    #[error("查询错误: {0}")]
    Query(String),
    #[error("查询错误 [{db_type}]: {message}\nSQL: {sql}")]
    QueryWithContext {
        db_type: String,
        message: String,
        sql: String,
    },
    #[error("连接池错误: {0}")]
    Pool(String),
}

#[allow(dead_code)] // 公开 API，供外部使用
impl DbError {
    /// 创建带数据库类型的连接错误
    pub fn connection_typed(db_type: &DatabaseType, message: impl Into<String>) -> Self {
        Self::ConnectionTyped {
            db_type: db_type.display_name().to_string(),
            message: message.into(),
        }
    }

    /// 创建带上下文的查询错误
    pub fn query_with_context(
        db_type: &DatabaseType,
        message: impl Into<String>,
        sql: &str,
    ) -> Self {
        // 截断 SQL 以避免日志过长
        let sql_preview = if sql.len() > 200 {
            format!("{}...", &sql[..200])
        } else {
            sql.to_string()
        };
        Self::QueryWithContext {
            db_type: db_type.display_name().to_string(),
            message: message.into(),
            sql: sql_preview,
        }
    }
}
