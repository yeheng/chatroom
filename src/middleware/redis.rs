use lazy_static::lazy_static;
use redis::Client;

use crate::config::CONFIG;

lazy_static! {
    pub static ref REDIS: Redis = Redis::default();
}

pub struct Redis {
    pub client: Client,
}

impl Default for Redis {
    fn default() -> Self {
        let url = CONFIG.redis.url.clone();
        let client = Client::open(url).unwrap();
        Redis { client }
    }
}


#[macro_export]
macro_rules! redis_conn {
    () => {
        $crate::middleware::redis::REDIS.client.get_connection().unwrap()
    };
}
