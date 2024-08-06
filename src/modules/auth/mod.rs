pub mod controller;
pub mod model;
pub mod service;
use lazy_static::lazy_static;

pub use service::AuthService as AuthService;

lazy_static! {
    pub static ref Service: AuthService = AuthService{};
}
