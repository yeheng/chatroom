use crate::websocket::server::ChatServerHandle;
use actix_web::{web, Error, HttpRequest, Responder};
use tokio::task::spawn_local;

pub mod model;
pub mod server;
pub mod handler;

// 注册一个新的WebSocket处理程序,该处理程序将使用WsChatSession作为会话处理程序。
pub async fn chat_ws(req: HttpRequest,
                     stream: web::Payload,
                     chat_server: web::Data<ChatServerHandle>) -> Result<impl Responder, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    spawn_local(handler::chat_ws(
        (**chat_server).clone(),
        session,
        msg_stream,
    ));

    Ok(res)
}
