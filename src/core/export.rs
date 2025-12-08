use crate::database::QueryResult;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    Csv,
    Sql,
    Json,
}

impl ExportFormat {
    pub fn extension(&self) -> &str {
        match self {
            ExportFormat::Csv => "csv",
            ExportFormat::Sql => "sql",
            ExportFormat::Json => "json",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            ExportFormat::Csv => "CSV",
            ExportFormat::Sql => "SQL",
            ExportFormat::Json => "JSON",
        }
    }
}

pub fn export_to_csv(result: &QueryResult, path: &Path) -> Result<(), String> {
    let mut file = File::create(path).map_err(|e| e.to_string())?;

    // 写入列头
    let header = result
        .columns
        .iter()
        .map(|c| escape_csv_field(c))
        .collect::<Vec<_>>()
        .join(",");
    writeln!(file, "{}", header).map_err(|e| e.to_string())?;

    // 写入数据行
    for row in &result.rows {
        let line = row
            .iter()
            .map(|cell| escape_csv_field(cell))
            .collect::<Vec<_>>()
            .join(",");
        writeln!(file, "{}", line).map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn export_to_sql(result: &QueryResult, table_name: &str, path: &Path) -> Result<(), String> {
    let mut file = File::create(path).map_err(|e| e.to_string())?;

    writeln!(file, "-- Exported from Rust DB Manager").map_err(|e| e.to_string())?;
    writeln!(file, "-- Table: {}", table_name).map_err(|e| e.to_string())?;
    writeln!(file, "-- Rows: {}\n", result.rows.len()).map_err(|e| e.to_string())?;

    if result.columns.is_empty() || result.rows.is_empty() {
        writeln!(file, "-- No data to export").map_err(|e| e.to_string())?;
        return Ok(());
    }

    let columns_str = result
        .columns
        .iter()
        .map(|c| format!("`{}`", c))
        .collect::<Vec<_>>()
        .join(", ");

    for row in &result.rows {
        let values = row
            .iter()
            .map(|cell| {
                if cell == "NULL" {
                    "NULL".to_string()
                } else {
                    format!("'{}'", cell.replace('\'', "''"))
                }
            })
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(
            file,
            "INSERT INTO `{}` ({}) VALUES ({});",
            table_name, columns_str, values
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn export_to_json(result: &QueryResult, path: &Path) -> Result<(), String> {
    let mut file = File::create(path).map_err(|e| e.to_string())?;

    let json_rows: Vec<serde_json::Map<String, serde_json::Value>> = result
        .rows
        .iter()
        .map(|row| {
            result
                .columns
                .iter()
                .zip(row.iter())
                .map(|(col, cell)| {
                    let value = if cell == "NULL" {
                        serde_json::Value::Null
                    } else if let Ok(num) = cell.parse::<i64>() {
                        serde_json::Value::Number(num.into())
                    } else if let Ok(num) = cell.parse::<f64>() {
                        serde_json::json!(num)
                    } else {
                        serde_json::Value::String(cell.clone())
                    };
                    (col.clone(), value)
                })
                .collect()
        })
        .collect();

    let json = serde_json::to_string_pretty(&json_rows).map_err(|e| e.to_string())?;
    write!(file, "{}", json).map_err(|e| e.to_string())?;

    Ok(())
}

fn escape_csv_field(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

pub fn import_sql_file(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}
