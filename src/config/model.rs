use std::collections::HashMap;
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    // 数据库连接 URL
    pub url: String,
    // 连接池大小
    pub pool_size: usize,
    // 连接池超时时间
    pub pool_timeout: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Redis {
    // Redis 连接 URL
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwt {
    // JWT 密钥
    pub secret: String,
    // JWT 过期时间
    pub exp: usize,
    // 刷新令牌过期时间
    pub refresh_token: usize,
    // 编码密钥
    pub encode_key: String,
    // 解码密钥
    pub decode_key: String,
    // 签发者
    pub issuer: String,
}

/// 配置
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationConfig {
    // 端口号
    pub port: u16,
    // 主机地址
    pub host: String,
    // 数据库配置
    pub database: Database,
    // JWT 配置
    pub jwt: Jwt,
    // Redis 配置
    pub redis: Redis,
    // 日志级别
    pub log_level: String,
    // 密钥
    pub key: String,
    // 白名单 API 列表
    pub white_list_api: Vec<String>,
    // 错误信息映射
    pub errors: HashMap<String, String>,
    // 错误信息详情（可选）
    pub error_infos: Option<HashMap<String, String>>,
}

#[derive(Clone)]
pub struct JwtKey {
    // 编码密钥
    pub encoding_key: EncodingKey,
    // 解码密钥
    pub decoding_key: DecodingKey,
}
