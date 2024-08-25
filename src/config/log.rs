use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

use crate::config::CONFIG;

pub fn init_log() {
    // 创建一个日志订阅器注册表
    tracing_subscriber::registry()
        // 尝试从环境变量中获取日志过滤器,如果失败则使用配置文件中的日志级别
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&CONFIG.log_level)))
        // 添加错误处理层
        .with(ErrorLayer::default())
        // 添加格式化层,并将输出写入标准输出
        .with(fmt::layer().with_writer(std::io::stdout))
        // 初始化日志系统
        .init();
}