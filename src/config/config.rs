use std::collections::HashMap;
use std::env;

use config::{Config, Environment, File};

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Database {
    pub url: String,
    pub pool_size: usize,
    pub pool_timeout: usize,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Log {
    pub dir: String,
    pub rolling: String,
    pub pack_compress: String,
    pub keep_type: String,
    pub level: String,
    pub chan_len: Option<usize>,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Jwt {
    pub secret: String,
    pub exp: usize,
    pub refresh_token: usize,
}

/// Config
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    pub port: u16,
    pub host: String,
    pub database: Database,
    pub log: Log,
    pub jwt: Jwt,

    pub sms_cache_send_key_prefix: String,
    pub white_list_api: Vec<String>,
    pub cache: String,
    pub login_fail_retry: u64,
    pub login_fail_retry_wait_sec: u64,
    pub trash_recycle_days: u64,
    pub datetime_format: String,
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
            .add_source(
                File::with_name(&format!("etc/config/{}", run_mode))
                    .required(false),
            )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("etc/local").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            .build()
            .unwrap();

        // Now that we're done, let's access our configuration
        println!("database: {:?}", s.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize().unwrap()
    }
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        ApplicationConfig::new()
    }
}

impl ApplicationConfig {
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

#[macro_export]
macro_rules! error_info {
    ($code: expr) => {
        $crate::service::CONTEXT.config.get_error_info($code)
    };
}
