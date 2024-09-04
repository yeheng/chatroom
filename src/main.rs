use actix_web::{App, HttpServer};

use chatroom::config::{app_log, CONFIG};

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::web::{self};
use actix_web_lab::middleware::NormalizePath;
use chatroom::util::error::CustomError;
use chatroom::AppState;
use tracing_actix_web::TracingLogger;

use chatroom::{modules, websocket};

// 主函数
#[actix_web::main]
async fn main() {
    // 初始化日志
    app_log::init_log();

    // 获取服务器实例
    // 格式化服务器地址
    let address = format!("{}:{}", &CONFIG.host, &CONFIG.port);

    // 记录服务器运行地址
    log::info!(
        "Server running at http://{}",
        format!("{}:{}", CONFIG.host, CONFIG.port)
    );

    // 创建应用状态
    let state = AppState::new().await;

    // 创建并配置 HTTP 服务器
    HttpServer::new(move || {
        App::new()
            // 添加应用状态
            .app_data(web::Data::new(state.clone()))
            // 设置 JSON 配置,限制请求体大小
            .app_data(
                web::JsonConfig::default()
                    .limit(4096)
                    .error_handler(|err, _req| {
                        CustomError::ValidationError {
                            field: err.to_string(),
                        }
                        .into()
                    }),
            )
            // 添加中间件
            .wrap(TracingLogger::default()) // 追踪日志
            .wrap(Logger::default()) // 默认日志
            .wrap(Cors::permissive()) // CORS 策略
            .wrap(DefaultHeaders::new().add(header::ContentType::json())) // 默认响应头
            .wrap(NormalizePath::trim()) // 规范化路径
            // 添加 WebSocket 路由
            .service(web::resource("/ws").to(websocket::chat_ws))
            // 配置其他路由
            .configure(modules::route::router)
    })
    // 绑定服务器地址
    .bind(address.clone())
    .expect(&format!("Can not bind to {}", address))
    // 运行服务器
    .run()
    .await
    .expect("Failed to run server");
}
