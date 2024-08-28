use redis::Client;
use sea_orm::DatabaseConnection;

pub mod config;
pub mod middleware;
pub mod model;
pub mod modules;
pub mod util;
pub mod websocket;

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection,
    redis: Client,
}

impl AppState {
    pub async fn new() -> Self {
        let conn = middleware::datasource::connect().await;
        let redis = middleware::redis::connect();
        Self { conn, redis }
    }

    pub fn get_conn(&self) -> &DatabaseConnection {
        &self.conn
    }

    pub fn get_redis(&self) -> &Client {
        &self.redis
    }
}
