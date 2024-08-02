use rbatis::RBatis;
use rbs::to_value;
use redis::{Commands, Connection};

use chatroom::config::app_log;
use chatroom::config::config::CONFIG;
use chatroom::middleware::datasource::{DATA_SOURCE, init_database};
use chatroom::middleware::redis::REDIS;
use chatroom::redis_client;

#[tokio::main]
async fn main() {
    app_log::init_log();
    init_database().await;
    log::info!("{:?}", CONFIG);

    let rb: &RBatis = &DATA_SOURCE.rb;
    let count: i32 = rb.query_decode(r#"select 1"#, vec![to_value!()]).await.unwrap();

    log::info!("count = {}", count);

    let mut conn: Connection = redis_client!().get_connection().unwrap();
    let _: () = conn.set("my_key", 42).unwrap();
    let value: i32 = conn.get("my_key").unwrap();
    log::info!("my_key = {}", value);
}
