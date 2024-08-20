use once_cell::sync::Lazy;
use redis::Client;

use crate::config::CONFIG;

// 定义一个全局的 Redis 实例
pub static REDIS: Lazy<Redis> = Lazy::new(|| Redis::default());

pub struct Redis {
    pub client: Client,
}

impl Default for Redis {
    fn default() -> Self {
        // 从配置中获取 Redis 的 URL 并创建客户端
        let url = CONFIG.redis.url.clone();
        let client = Client::open(url).unwrap();
        Redis { client }
    }
}

#[macro_export]
macro_rules! redis_conn {
    () => {
        // 获取 Redis 连接
        $crate::middleware::redis::REDIS
            .client
            .get_connection()
            .unwrap()
    };
}
