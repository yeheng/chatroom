extern crate rbatis;

use actix_web::{App, HttpServer};

use chatroom::config::{app_log, CONFIG};
use chatroom::middleware::datasource::DB;

use std::ops::Deref;

use actix_cors::Cors;
use actix_http::StatusCode;
use actix_web::dev::Server;
use actix_web::http::header;
use actix_web::middleware::{DefaultHeaders, ErrorHandlers, Logger};
use actix_web::web;
use actix_web_lab::middleware::NormalizePath;
use tracing_actix_web::TracingLogger;

use chatroom::{middleware, modules, websocket};

pub async fn get_server() -> Server {
    let config = CONFIG.deref();
    let address = format!("{}:{}", config.host, config.port);

    log::info!(
        "Server running at http://{}",
        format!("{}:{}", CONFIG.host, CONFIG.port)
    );
    HttpServer::new(|| {
        App::new()
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

    DB.init_database().await;

    let server = get_server();
    server.await.await.expect("server start failed");
}
