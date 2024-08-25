use redis::Client;

use crate::config::CONFIG;

pub fn connect() -> Client {
    // 从配置中获取 Redis 的 URL 并创建客户端
    let url = CONFIG.redis.url.clone();
    Client::open(url).unwrap()
}
