use std::time::Duration;

use crate::config::CONFIG;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

/// 连接到数据库
///
/// # Returns
///
/// * `DatabaseConnection` - 数据库连接实例
pub async fn connect() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(&CONFIG.database.url);
    opt.max_connections(CONFIG.database.pool_size as u32) // 设置最大连接数
        .min_connections(5) // 设置最小连接数
        .connect_timeout(Duration::from_secs(8)) // 设置连接超时时间
        .acquire_timeout(Duration::from_secs(8)) // 设置获取连接超时时间
        .idle_timeout(Duration::from_secs(8)) // 设置空闲连接超时时间
        .max_lifetime(Duration::from_secs(CONFIG.database.pool_timeout as u64)) // 设置连接最大存活时间
        .sqlx_logging(true) // 启用 SQLx 日志记录
        .sqlx_logging_level(log::LevelFilter::Info); // 设置 SQLx 日志记录级别
    let r = Database::connect(opt).await;
    match r {
        Ok(_) => r.unwrap(),
        Err(e) => {
            log::error!("Failed to connect to the database, reason: {:?}", e); // 记录连接失败的错误信息
            std::process::exit(1); // 退出程序
        }
    }
}