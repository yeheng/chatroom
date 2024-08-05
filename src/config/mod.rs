use jsonwebtoken::{DecodingKey, EncodingKey};
use lazy_static::lazy_static;
pub use log as app_log;
use crate::config::config::{ApplicationConfig, JwtKey};

pub mod config;
pub mod log;

lazy_static! {
    pub static ref CONFIG: ApplicationConfig = ApplicationConfig::new();
    pub static ref JWT_KEY: JwtKey = JwtKey {
        encoding_key: EncodingKey::from_ed_pem(CONFIG.jwt.encode_key.as_bytes()).unwrap(),
        decoding_key: DecodingKey::from_ed_pem(CONFIG.jwt.decode_key.as_bytes()).unwrap(),
    };
}
