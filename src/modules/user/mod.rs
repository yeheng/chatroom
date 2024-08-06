pub mod controller;
pub mod service;
pub mod model;

use lazy_static::lazy_static;

pub use service::UserService as UserService;

lazy_static! {
    pub static ref Service: UserService = UserService{};
}
