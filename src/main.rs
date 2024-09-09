use actix_web::{App, HttpServer};

use actix_web_httpauth::middleware::HttpAuthentication;
use chatroom::config::{app_log, CONFIG};

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::web::{self};
use actix_web_lab::middleware::NormalizePath;
use chatroom::util::error::CustomError;
use chatroom::{middleware, AppState};
use tokio::task::spawn;
use tokio::try_join;
use tracing_actix_web::TracingLogger;

use chatroom::websocket::server::ChatServer;
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

    let (chat_server, server_tx) = ChatServer::new();

    let chat_server = spawn(chat_server.run());

    // 创建并配置 HTTP 服务器
    let http_server = HttpServer::new(move || {
        App::new()
            // 添加应用状态
            .app_data(web::Data::new(state.clone()))
            .app_data(web::Data::new(server_tx.clone()))
            // 设置 JSON 配置,限制请求体大小
            .app_data(
                web::JsonConfig::default()
                    .limit(4096)
                    .error_handler(|err, _req| {
                        CustomError::ValidationError {
                            message: format!("JSON validation error: {}", err),
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
            .service(
                web::resource("/ws")
                    .wrap(HttpAuthentication::with_fn(middleware::auth::validator))
                    .to(websocket::chat_ws),
            )
            // 配置其他路由
            .configure(modules::route::router)
    })
    // 绑定服务器地址
    .bind(address.clone())
    .map_err(|e| {
        log::error!("Failed to bind to {}: {}", address, e);
        std::process::exit(1);
    })
    .expect("REASON")
    // 运行服务器
    .run();

    try_join!(http_server, async move { chat_server.await.unwrap() })
        .map_err(|e| {
            log::error!("Failed to start server: {}", e);
            std::process::exit(1);
        })
        .unwrap();
}
