use actix_web::web::ServiceConfig;
use controller::login;

mod controller;
pub mod model;

pub fn router(config: &mut ServiceConfig) {
    config.service(login);
}