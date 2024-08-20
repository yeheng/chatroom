use crate::config::model::{ApplicationConfig, JwtKey};
use jsonwebtoken::{DecodingKey, EncodingKey};
pub use log as app_log;
use once_cell::sync::Lazy;

pub mod config;
pub mod log;
pub mod model;

pub static CONFIG: Lazy<ApplicationConfig> = Lazy::new(|| ApplicationConfig::new());
pub static JWT_KEY: Lazy<JwtKey> = Lazy::new(|| JwtKey {
    encoding_key: EncodingKey::from_ed_pem(CONFIG.jwt.encode_key.as_bytes()).unwrap(),
    decoding_key: DecodingKey::from_ed_pem(CONFIG.jwt.decode_key.as_bytes()).unwrap(),
});
