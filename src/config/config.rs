use std::collections::HashMap;
use std::env;

use crate::config::model::ApplicationConfig;
use config::{Config, Environment, File};

impl ApplicationConfig {
    // 创建新的ApplicationConfig实例
    pub fn new() -> Self {
        // 从环境变量获取运行模式,默认为"dev"
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());

        // 构建配置
        let s = Config::builder()
            // 首先合并"default"配置文件
            .add_source(File::with_name("etc/default"))
            .add_source(File::with_name("/etc/default").required(false))
            // 添加当前环境的配置文件
            // 默认使用'development'环境
            // 注意：这个文件是可选的
            .add_source(File::with_name(&format!("etc/config/{}", run_mode)).required(false))
            .add_source(File::with_name(&format!("/etc/config/{}", run_mode)).required(false))
            // 添加本地配置文件
            // 这个文件不应该被提交到git
            .add_source(File::with_name("etc/local").required(false))
            // 从环境变量添加设置（使用APP前缀）
            // 例如：`APP_DEBUG=1 ./target/app` 会设置 `debug` 键
            .add_source(Environment::with_prefix("app"))
            .build()
            .unwrap();

        // 反序列化配置
        s.try_deserialize().unwrap()
    }

    // 根据错误代码获取错误信息
    pub fn get_error_info(&self, code: &str) -> String {
        match self.errors.get(code) {
            None => match self.errors.get("-1") {
                None => "unknown error".to_string(),
                Some(v) => v.to_string(),
            },
            Some(v) => v.as_str().to_string(),
        }
    }

    // 初始化错误信息
    pub fn init_infos(&mut self) {
        self.error_infos = Some(HashMap::new());
        for (k, error) in &self.errors {
            let mut error = error.to_string();
            // 如果错误信息包含逗号,只取逗号前的部分
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

// 为ApplicationConfig实现Default trait
impl Default for ApplicationConfig {
    fn default() -> Self {
        ApplicationConfig::new()
    }
}

// 定义一个宏来获取错误信息
#[macro_export]
macro_rules! error_info {
    ($code: expr) => {
        $crate::config::CONFIG.get_error_info($code)
    };
}
