use actix_web::web::ServiceConfig;
use controller::login;
use once_cell::sync::Lazy;
use service::AuthService;

mod controller;
pub mod model;
pub mod service;

pub static SERVICE: Lazy<AuthService> = Lazy::new(|| AuthService{});

pub fn router(config: &mut ServiceConfig) {
    config.service(login);
}