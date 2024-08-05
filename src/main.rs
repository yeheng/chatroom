
use rbs::to_value;
use redis::{Commands, Connection};

use chatroom::config::app_log;
use chatroom::server::Application;

#[tokio::main]
async fn main() {
    app_log::init_log();

    let app = Application::build()
        .await;

    let count: i32 = app.rb.query_decode(r#"select 1"#, vec![to_value!()]).await.unwrap();
    log::info!("count = {}", count);

    let mut conn: Connection = app.client.get_connection().unwrap();
    let _: () = conn.set("my_key", 42).unwrap();
    let value: i32 = conn.get("my_key").unwrap();
    log::info!("my_key = {}", value);

    app.run_until_stopped().await.unwrap();

}
