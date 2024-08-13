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
}
