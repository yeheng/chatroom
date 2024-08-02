use std::collections::HashMap;
use std::env;

use config::{Config, Environment, File};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static CONFIG: Lazy<ApplicationConfig> = Lazy::new(ApplicationConfig::default);

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub url: String,
    pub db_name: String,
    pub pool_size: usize,
    pub pool_timeout: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Redis {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwt {
    pub secret: String,
    pub exp: usize,
    pub refresh_token: usize,
    pub encode_key: String,
    pub decode_key: String,
}

/// Config
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub port: u16,
    pub host: String,
    pub database: Database,
    pub jwt: Jwt,
    pub redis: Redis,

    pub white_list_api: Vec<String>,
    pub errors: HashMap<String, String>,
    pub error_infos: Option<HashMap<String, String>>,
}

impl ApplicationConfig {
    pub fn new() -> Self {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("etc/default"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(File::with_name(&format!("etc/config/{}", run_mode)).required(false))
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("etc/local").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            .build()
            .unwrap();

        return s.try_deserialize().unwrap();
    }

    pub fn get_error_info(&self, code: &str) -> String {
        match self.errors.get(code) {
            None => match self.errors.get("-1") {
                None => "unknown error".to_string(),
                Some(v) => v.to_string(),
            },
            Some(v) => v.as_str().to_string(),
        }
    }

    pub fn init_infos(&mut self) {
        self.error_infos = Some(HashMap::new());
        for (k, error) in &self.errors {
            let mut error = error.to_string();
            if error.contains(",") {
                error = error[0..error.find(",").unwrap()].to_string();
            }
            self.error_infos
                .as_mut()
                .unwrap()
                .insert(error, k.to_string());
        }
    }
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        ApplicationConfig::new()
    }
}

#[macro_export]
macro_rules! error_info {
    ($code: expr) => {
        $crate::service::CONTEXT.config.get_error_info($code)
    };
}
