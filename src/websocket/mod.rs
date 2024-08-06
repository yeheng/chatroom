pub mod model;
pub mod server;
pub mod session;

use actix_web::{web, Error, HttpRequest, Responder};
use actix_web_actors::ws;

use session::WsChatSession;

// 注册一个新的WebSocket处理程序，该处理程序将使用WsChatSession作为会话处理程序。
pub async fn chat_ws(req: HttpRequest, stream: web::Payload) -> Result<impl Responder, Error> {
    ws::start(WsChatSession::default(), &req, stream)
}
