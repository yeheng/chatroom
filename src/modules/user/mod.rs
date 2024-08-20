mod controller;
pub mod model;
pub mod service;

use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;
use controller::*;
pub use service::UserService;

use crate::middleware;

pub fn router(config: &mut ServiceConfig) {
    config.service(
        web::scope("/api").service(
            web::scope("/user")
                .wrap(HttpAuthentication::with_fn(middleware::auth::validator))
                .service(get_user_info)
                .service(get_current_user)
                .service(update_user),
        ),
    );
}
