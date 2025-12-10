use rust_db_manager::database::{ConnectionConfig, DatabaseType, connect_database};

#[tokio::main]
async fn main() {
    // 测试 MySQL 连接
    println!("=== 测试 MySQL 连接 ===");
    let mysql_config = ConnectionConfig {
        name: "guyong".to_string(),
        db_type: DatabaseType::MySQL,
        host: "111.229.127.6".to_string(),
        port: 3306,
        username: "root".to_string(),
        password: "Chen1234#".to_string(),
        database: String::new(),
        ..Default::default()
    };
    
    println!("连接字符串: mysql://root:****@111.229.127.6:3306");
    match connect_database(&mysql_config).await {
        Ok(result) => {
            println!("✓ MySQL 连接成功!");
            match result {
                rust_db_manager::database::ConnectResult::Databases(dbs) => {
                    println!("  数据库列表: {:?}", dbs);
                }
                rust_db_manager::database::ConnectResult::Tables(tables) => {
                    println!("  表列表: {:?}", tables);
                }
            }
        }
        Err(e) => {
            println!("✗ MySQL 连接失败: {}", e);
        }
    }
    
    println!("\n=== 测试 PostgreSQL 连接 ===");
    let pg_config = ConnectionConfig {
        name: "guyong".to_string(),
        db_type: DatabaseType::PostgreSQL,
        host: "111.229.127.6".to_string(),
        port: 5432,
        username: "root".to_string(),
        password: "Chen1234#".to_string(),
        database: "postgres".to_string(),
        ..Default::default()
    };
    
    println!("连接字符串: postgres://root:****@111.229.127.6:5432/postgres");
    match connect_database(&pg_config).await {
        Ok(result) => {
            println!("✓ PostgreSQL 连接成功!");
            match result {
                rust_db_manager::database::ConnectResult::Databases(dbs) => {
                    println!("  数据库列表: {:?}", dbs);
                }
                rust_db_manager::database::ConnectResult::Tables(tables) => {
                    println!("  表列表: {:?}", tables);
                }
            }
        }
        Err(e) => {
            println!("✗ PostgreSQL 连接失败: {}", e);
        }
    }
}
