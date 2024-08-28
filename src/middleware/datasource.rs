use std::time::Duration;

use crate::config::CONFIG;
use fastdate::DurationFrom;
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
        .connect_timeout(parse_duration(&CONFIG.database.connect_timeout)) // 设置连接超时时间
        .acquire_timeout(parse_duration(&CONFIG.database.acquire_timeout)) // 设置获取连接超时时间
        .idle_timeout(parse_duration(&CONFIG.database.idle_timeout)) // 设置空闲连接超时时间
        .max_lifetime(parse_duration(&CONFIG.database.pool_timeout)) // 设置连接最大存活时间
        .sqlx_logging(CONFIG.database.sqlx_logging) // 启用 SQLx 日志记录
        .sqlx_logging_level(parse_log_level(&CONFIG.database.sqlx_logging_level)); // 设置 SQLx 日志记录级别
    let r = Database::connect(opt).await;
    match r {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to connect to the database, reason: {:?}", e); // 记录连接失败的错误信息
            std::process::exit(1); // 退出程序
        }
    }
}

fn parse_log_level(sqlx_logging_level: &str) -> log::LevelFilter {
    match sqlx_logging_level.to_lowercase().as_str() {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Off,
    }
}

fn parse_duration(log_rolling: &str) -> Duration {
    let lower = log_rolling.to_lowercase();
    let unit = lower.chars().last().unwrap();
    let number = lower.trim_end_matches(unit).parse::<u64>().unwrap_or_else(|_| {
        panic!("Invalid duration format: {}", log_rolling);
    });
    let duration = match unit.to_string().as_str() {
        "d" => Duration::from_day(number),
        "h" => Duration::from_hour(number),
        "m" => Duration::from_minute(number),
        _ => Duration::from_secs(number),
    };
    duration
}
