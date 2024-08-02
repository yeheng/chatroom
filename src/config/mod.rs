use once_cell::sync::Lazy;

use crate::config::config::ApplicationConfig;

pub mod datasource;
pub mod config;

pub static CONFIG: Lazy<ApplicationConfig> = Lazy::new(ApplicationConfig::default);
