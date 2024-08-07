extern crate rbatis;

use rbs::to_value;
use redis::{Commands, Connection};

use chatroom::config::app_log;
use chatroom::middleware::datasource::DB;
use chatroom::middleware::redis::REDIS;
use chatroom::server::Application;

#[actix_web::main]
async fn main() {
    app_log::init_log();

    let app = Application::build().await;

    DB.init_database().await;

    let count: i32 = DB
        .rb
        .query_decode(r#"select count(0) from sys_user"#, vec![to_value!()])
        .await
        .unwrap();
    log::info!("count = {}", count);

    let mut conn: Connection = REDIS.client.get_connection().unwrap();
    let _: () = conn.set("my_key", 42).unwrap();
    let value: i32 = conn.get("my_key").unwrap();
    log::info!("my_key = {}", value);

    app.run_until_stopped().await.unwrap();
}
