use std::ops::Deref;

use actix_cors::Cors;
use actix_http::StatusCode;
use actix_web::dev::Server;
use actix_web::http::header;
use actix_web::middleware::{DefaultHeaders, ErrorHandlers, Logger};
use actix_web::{web, App, HttpServer};
use actix_web_lab::middleware::NormalizePath;
use tracing_actix_web::TracingLogger;

use crate::config::CONFIG;
use crate::{middleware, modules, websocket};

pub struct Application {
    pub server: Server,
}

impl Application {
    pub async fn build() -> Self {
        let config = CONFIG.deref();
        let address = format!("{}:{}", config.host, config.port);

        let server = HttpServer::new(|| {
            App::new()
                .app_data(web::JsonConfig::default().limit(4096))
                .wrap(TracingLogger::default())
                .wrap(Logger::default())
                .wrap(Cors::permissive())
                .wrap(DefaultHeaders::new().add(header::ContentType::json()))
                .wrap(NormalizePath::trim())
                .wrap(
                    ErrorHandlers::new()
                        .handler(StatusCode::BAD_REQUEST, middleware::format_response),
                )
                .service(web::resource("/ws").to(websocket::chat_ws))
                .configure(modules::route::router)
        })
        .bind(address.clone())
        .expect(&format!("Can not bind to {}", address))
        .run();

        Application { server }
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        log::info!(
            "Server running at http://{}",
            format!("{}:{}", CONFIG.host, CONFIG.port)
        );
        self.server.await
    }
}
