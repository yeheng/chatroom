use once_cell::sync::Lazy;
use redis::Commands;

use crate::config::config::CONFIG;

pub static REDIS: Lazy<Redis> = Lazy::new(Redis::default);

pub struct Redis {
    pub client: redis::Client,
}
impl Default for Redis {
    fn default() -> Self {
        let url = CONFIG.redis.url.clone();
        let client = redis::Client::open(url).unwrap();

        let mut conn = client.get_connection().unwrap();
        let _v: bool = conn.exists("my_key").unwrap();
        log::info!("redis init ({})...", CONFIG.redis.url);
        Self {
            client,
        }
    }
}

#[macro_export]
macro_rules! redis_client {
    () => {
        &REDIS.client
    };
}
