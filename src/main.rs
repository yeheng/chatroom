use actix_web::{App, HttpServer};

use chatroom::config::{app_log, CONFIG};

use actix_cors::Cors;
use actix_http::StatusCode;
use actix_web::dev::Server;
use actix_web::http::header;
use actix_web::middleware::{DefaultHeaders, ErrorHandlers, Logger};
use actix_web::web::{self};
use actix_web_lab::middleware::NormalizePath;
use chatroom::AppState;
use tracing_actix_web::TracingLogger;

use chatroom::{middleware, modules, websocket};

pub async fn get_server() -> Server {
    let address = format!("{}:{}", &CONFIG.host, &CONFIG.port);

    log::info!(
        "Server running at http://{}",
        format!("{}:{}", CONFIG.host, CONFIG.port)
    );

    let conn = middleware::datasource::connect().await;
    let state = AppState { conn };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .app_data(web::JsonConfig::default().limit(4096))
            .wrap(TracingLogger::default())
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .wrap(DefaultHeaders::new().add(header::ContentType::json()))
            .wrap(NormalizePath::trim())
            .wrap(
                ErrorHandlers::new().handler(StatusCode::BAD_REQUEST, middleware::format_response),
            )
            .service(web::resource("/ws").to(websocket::chat_ws))
            .configure(modules::route::router)
    })
    .bind(address.clone())
    .expect(&format!("Can not bind to {}", address))
    .run()
}

#[actix_web::main]
async fn main() {
    app_log::init_log();

    let server = get_server();
    server.await.await.expect("server start failed");
}
