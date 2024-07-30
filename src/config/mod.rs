use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;

use crate::config::config::ApplicationConfig;

pub mod datasource;
pub mod config;

pub static CONFIG: Lazy<ApplicationConfig> = Lazy::new(ApplicationConfig::default);
pub static ED25519_PRIVATE_KEY: Lazy<EncodingKey> = Lazy::new(|| EncodingKey::from_ed_pem(include_bytes!("../../realworld.pem")).expect("Not a valid EdDSA private key!"));
pub static ED25519_PUBLIC_KEY: Lazy<DecodingKey> = Lazy::new(|| DecodingKey::from_ed_pem(include_bytes!("../../realworld.pub")).expect("Not a valid EdDSA public key!"));
