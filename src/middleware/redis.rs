use redis::{Client, Commands};

use crate::config::config::ApplicationConfig;

pub fn init_redis(config: &ApplicationConfig) -> Client {
    let url = config.redis.url.clone();
    let client = redis::Client::open(url).unwrap();

    let mut conn = client.get_connection().unwrap();
    let _v: bool = conn.exists("my_key").unwrap();
    log::info!("redis init ({})...", config.redis.url);
    client
}
