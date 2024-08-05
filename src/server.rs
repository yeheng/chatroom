use std::ops::Deref;

use actix_cors::Cors;
use actix_http::StatusCode;
use actix_web::{App, HttpServer};
use actix_web::dev::Server;
use actix_web::http::header;
use actix_web::middleware::{Compress, DefaultHeaders, ErrorHandlers, Logger};
use actix_web_lab::middleware::NormalizePath;
use rbatis::RBatis;
use tracing_actix_web::TracingLogger;

use crate::config::CONFIG;
use crate::middleware;
use crate::middleware::datasource::init_database;
use crate::middleware::redis::init_redis;

pub struct Application {
    pub server: Server,
    pub rb: RBatis,
    pub client: redis::Client,
}

impl Application {
    pub async fn build() -> Self {
        let config = CONFIG.deref();
        let address = format!(
            "{}:{}",
            config.host, config.port
        );
        let server = HttpServer::new(|| {
            App::new()
                .wrap(TracingLogger::default())
                .wrap(Logger::default())
                .wrap(Cors::permissive())
                .wrap(DefaultHeaders::new()
                    .add(header::ContentType::json())
                )
                .wrap(Compress::default())
                .wrap(NormalizePath::trim())
                .wrap(
                    ErrorHandlers::new()
                        .handler(StatusCode::BAD_REQUEST,
                                 middleware::format_response)
                )
        })
            .bind(address.clone())
            .expect("Can not bind to port 8080")
            .run();

        let rb = init_database(&config).await;
        let client = init_redis(&config);

        Application {
            server,
            rb,
            client,
        }
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        log::info!("Server running at http://{}", format!("{}:{}",CONFIG.host, CONFIG.port));
        self.server.await
    }
}
