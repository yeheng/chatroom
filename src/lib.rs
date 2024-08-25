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
    pub conn: DatabaseConnection,
    pub redis: Client,
}

impl AppState {

    pub async fn new() -> Self {
        let conn = middleware::datasource::connect().await;
        let redis = middleware::redis::connect();
        Self { conn, redis }
    }

}