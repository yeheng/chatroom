use std::time::Duration;

use crate::config::CONFIG;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn connect() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(&CONFIG.database.url);
    opt.max_connections(CONFIG.database.pool_size as u32)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(CONFIG.database.pool_timeout as u64))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    let r = Database::connect(opt).await;
    match r {
        Ok(_) => r.unwrap(),
        Err(e) => {
            log::error!("Failed to connect to the database, reason: {:?}", e);
            std::process::exit(1);
        }
    }
}
