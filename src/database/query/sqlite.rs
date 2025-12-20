//! SQLite 查询实现

use rusqlite::{types::ValueRef, Connection as SqliteConn};
use crate::database::{ConnectionConfig, DbError, QueryResult, DatabaseType};
use super::{query_result, exec_result, is_query_statement, TriggerInfo, ForeignKeyInfo, ColumnInfo};

/// 连接 SQLite 并获取表列表
pub fn connect(config: &ConnectionConfig) -> Result<Vec<String>, DbError> {
    let conn = SqliteConn::open(&config.database)
        .map_err(|e| DbError::Connection(format!("SQLite 连接失败: {}", e)))?;

    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name"
    ).map_err(|e| DbError::Query(e.to_string()))?;

    let tables: Result<Vec<String>, _> = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| DbError::Query(e.to_string()))?
        .collect();

    tables.map_err(|e| DbError::Query(e.to_string()))
}

/// 获取 SQLite 表的主键列名
pub fn get_primary_key(config: &ConnectionConfig, table: &str) -> Result<Option<String>, DbError> {
    let conn = SqliteConn::open(&config.database)
        .map_err(|e| DbError::Connection(format!("SQLite 连接失败: {}", e)))?;
    
    // 使用 PRAGMA table_info 查询主键列（pk 字段 > 0 表示是主键）
    let escaped_table = table.replace('\'', "''");
    let sql = format!("PRAGMA table_info('{}')", escaped_table);
    
    let mut stmt = conn.prepare(&sql)
        .map_err(|e| DbError::Query(e.to_string()))?;
    
    // table_info 返回: cid, name, type, notnull, dflt_value, pk
    let pk_columns: Vec<String> = stmt
        .query_map([], |row| {
            let pk: i32 = row.get(5)?;  // pk 列
            let name: String = row.get(1)?;  // name 列
            Ok((name, pk))
        })
        .map_err(|e| DbError::Query(e.to_string()))?
        .filter_map(|r| r.ok())
        .filter(|(_, pk)| *pk > 0)
        .map(|(name, _)| name)
        .collect();
    
    // 返回第一个主键列（通常只有一个）
    Ok(pk_columns.into_iter().next())
}

/// 执行 SQLite 查询
pub fn execute(config: &ConnectionConfig, sql: &str) -> Result<QueryResult, DbError> {
    let conn = SqliteConn::open(&config.database)
        .map_err(|e| DbError::Connection(format!("SQLite 连接失败: {}", e)))?;

    if is_query_statement(sql, &DatabaseType::SQLite) {
        let mut stmt = conn
            .prepare(sql)
            .map_err(|e| DbError::Query(e.to_string()))?;

        let columns: Vec<String> = stmt.column_names().into_iter().map(String::from).collect();

        let rows: Result<Vec<Vec<String>>, _> = stmt
            .query_map([], |row| {
                (0..columns.len())
                    .map(|i| value_to_string(row.get_ref(i)))
                    .collect::<Result<Vec<_>, _>>()
            })
            .map_err(|e| DbError::Query(e.to_string()))?
            .collect();

        let rows = rows.map_err(|e| DbError::Query(e.to_string()))?;
        Ok(query_result(columns, rows))
    } else {
        let affected = conn
            .execute(sql, [])
            .map_err(|e| DbError::Query(e.to_string()))? as u64;
        Ok(exec_result(affected))
    }
}

/// 将 SQLite 值转换为字符串
fn value_to_string(
    val: Result<ValueRef<'_>, rusqlite::Error>,
) -> Result<String, rusqlite::Error> {
    Ok(match val? {
        ValueRef::Null => String::from("NULL"),
        ValueRef::Integer(i) => i.to_string(),
        ValueRef::Real(f) => f.to_string(),
        ValueRef::Text(t) => String::from_utf8_lossy(t).into_owned(),
        ValueRef::Blob(b) => format!("<Blob {} bytes>", b.len()),
    })
}

/// 获取 SQLite 触发器
pub fn get_triggers(config: &ConnectionConfig) -> Result<Vec<TriggerInfo>, DbError> {
    let conn = SqliteConn::open(&config.database)
        .map_err(|e| DbError::Connection(format!("SQLite 连接失败: {}", e)))?;

    let mut stmt = conn
        .prepare("SELECT name, tbl_name, sql FROM sqlite_master WHERE type='trigger' ORDER BY name")
        .map_err(|e| DbError::Query(e.to_string()))?;

    let triggers: Result<Vec<TriggerInfo>, _> = stmt
        .query_map([], |row| {
            let name: String = row.get(0)?;
            let table_name: String = row.get(1)?;
            let sql: String = row.get(2)?;

            // 从 SQL 中解析 timing 和 event
            let sql_upper = sql.to_uppercase();
            let timing = if sql_upper.contains("BEFORE") {
                "BEFORE"
            } else if sql_upper.contains("AFTER") {
                "AFTER"
            } else if sql_upper.contains("INSTEAD OF") {
                "INSTEAD OF"
            } else {
                "UNKNOWN"
            }
            .to_string();

            let event = if sql_upper.contains("INSERT") {
                "INSERT"
            } else if sql_upper.contains("UPDATE") {
                "UPDATE"
            } else if sql_upper.contains("DELETE") {
                "DELETE"
            } else {
                "UNKNOWN"
            }
            .to_string();

            Ok(TriggerInfo {
                name,
                table_name,
                event,
                timing,
                definition: sql,
            })
        })
        .map_err(|e| DbError::Query(e.to_string()))?
        .collect();

    triggers.map_err(|e| DbError::Query(e.to_string()))
}

/// 获取 SQLite 外键
pub fn get_foreign_keys(config: &ConnectionConfig) -> Result<Vec<ForeignKeyInfo>, DbError> {
    let conn = SqliteConn::open(&config.database)
        .map_err(|e| DbError::Connection(format!("SQLite 连接失败: {}", e)))?;

    // 首先获取所有表
    let mut tables_stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'")
        .map_err(|e| DbError::Query(e.to_string()))?;

    let tables: Vec<String> = tables_stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| DbError::Query(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    let mut foreign_keys = Vec::new();

    // 对每个表查询外键
    for table in tables {
        let sql = format!("PRAGMA foreign_key_list('{}')", table.replace('\'', "''"));
        let mut fk_stmt = conn
            .prepare(&sql)
            .map_err(|e| DbError::Query(e.to_string()))?;

        // foreign_key_list 返回: id, seq, table, from, to, on_update, on_delete, match
        let fks: Vec<ForeignKeyInfo> = fk_stmt
            .query_map([], |row| {
                let to_table: String = row.get(2)?;
                let from_column: String = row.get(3)?;
                let to_column: String = row.get(4)?;
                Ok(ForeignKeyInfo {
                    from_table: table.clone(),
                    from_column,
                    to_table,
                    to_column,
                })
            })
            .map_err(|e| DbError::Query(e.to_string()))?
            .filter_map(|r| r.ok())
            .collect();

        foreign_keys.extend(fks);
    }

    Ok(foreign_keys)
}

/// 获取 SQLite 表的列信息
pub fn get_columns(config: &ConnectionConfig, table: &str) -> Result<Vec<ColumnInfo>, DbError> {
    let conn = SqliteConn::open(&config.database)
        .map_err(|e| DbError::Connection(format!("SQLite 连接失败: {}", e)))?;

    let sql = format!("PRAGMA table_info('{}')", table.replace('\'', "''"));
    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| DbError::Query(e.to_string()))?;

    // table_info 返回: cid, name, type, notnull, dflt_value, pk
    let columns: Vec<ColumnInfo> = stmt
        .query_map([], |row| {
            let name: String = row.get(1)?;
            let data_type: String = row.get(2)?;
            let notnull: i32 = row.get(3)?;
            let default_value: Option<String> = row.get(4)?;
            let pk: i32 = row.get(5)?;
            Ok(ColumnInfo {
                name,
                data_type,
                is_primary_key: pk > 0,
                is_nullable: notnull == 0,
                default_value,
            })
        })
        .map_err(|e| DbError::Query(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(columns)
}
