use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ## 数据库配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Database {
    pub url: String,                // 数据库连接 URL
    pub pool_size: usize,           // 连接池大小
    pub pool_timeout: String,        // 连接池超时时间
    pub min_connections: usize,     // 设置最小连接数
    pub connect_timeout: String,    // 设置连接超时时间
    pub acquire_timeout: String,    // 设置获取连接超时时间
    pub idle_timeout: String,       // 设置空闲连接超时时间
    pub sqlx_logging: bool,         // 启用 SQLx 日志记录
    pub sqlx_logging_level: String, // 设置 SQLx 日志记录级别
}

/// ## Redis 配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Redis {
    // Redis 连接 URL
    pub url: String,
}

/// ## JWT 配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Jwt {
    pub secret: String,       // JWT 密钥
    pub exp: usize,           // JWT 过期时间
    pub refresh_token: usize, // 刷新令牌过期时间
    pub encode_key: String,   // 编码密钥
    pub decode_key: String,   // 解码密钥
    pub issuer: String,       // 签发者
}

/// ## App配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationConfig {
    pub port: u16,                                    // 端口号
    pub host: String,                                 // 主机地址
    pub database: Database,                           // 数据库配置
    pub jwt: Jwt,                                     // JWT 配置
    pub redis: Redis,                                 // Redis 配置
    pub log_level: String,                            // 日志级别
    pub key: String,                                  // 密钥
    pub white_list_api: Vec<String>,                  // 白名单 API 列表
    pub errors: HashMap<String, String>,              // 错误信息映射
    pub error_infos: Option<HashMap<String, String>>, // 错误信息详情（可选）
}

#[derive(Clone)]
pub struct JwtKey {
    pub encoding_key: EncodingKey, // 编码密钥
    pub decoding_key: DecodingKey, // 解码密钥
}
