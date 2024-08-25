use actix_web::{web, Error, HttpRequest, Responder};
use actix_web_actors::ws;
use once_cell::sync::Lazy;
use session::WsChatSession;
use std::{collections::HashMap, sync::Mutex};
use uuid::Uuid;

pub mod model;
pub mod server;
pub mod session;

pub static WS_SESSIONS: Lazy<Mutex<HashMap<String, WsChatSession>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

// 注册一个新的WebSocket处理程序,该处理程序将使用WsChatSession作为会话处理程序。
pub async fn chat_ws(req: HttpRequest, stream: web::Payload) -> Result<impl Responder, Error> {
    let id = Uuid::new_v4();

    let session = Box::new(WsChatSession::new(
        id,
        "main".to_owned(),
        Some("Anonymous".to_owned()),
    ));

    WS_SESSIONS
        .try_lock()
        .unwrap()
        .insert(id.to_string(), *session.clone());

    ws::start(*session.clone(), &req, stream)
}
