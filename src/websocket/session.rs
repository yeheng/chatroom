use actix::prelude::*;
use actix_broker::BrokerIssue;
use actix_web_actors::ws;
use uuid::Uuid;

use crate::websocket::{
    model::{ChatMessage, JoinRoom, LeaveRoom, ListRooms, SendMessage},
    server::WsChatServer,
};

#[derive(Default)]
pub struct WsChatSession {
    id: Uuid,
    room: String,
    name: Option<String>,
}

// `WsChatSession` 结构体表示 WebSocket 聊天会话。
impl WsChatSession {
    // 加入房间，如果需要则离开当前房间。
    pub fn join_room(&mut self, room_name: &str, ctx: &mut ws::WebsocketContext<Self>) {
        let room_name = room_name.to_owned();

        // 发送离开当前房间的消息。
        let leave_msg = LeaveRoom(self.room.clone(), self.id);
        self.issue_system_sync(leave_msg, ctx);

        // 发送加入新房间的消息。
        let join_msg = JoinRoom(
            room_name.to_owned(),
            self.name.clone(),
            ctx.address().recipient(),
        );

        // 更新会话的房间和 ID。
        WsChatServer::from_registry()
            .send(join_msg)
            .into_actor(self)
            .then(|id, act, _ctx| {
                if let Ok(id) = id {
                    act.id = id;
                    act.room = room_name;
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    // 列出所有房间。
    pub fn list_rooms(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        WsChatServer::from_registry()
            .send(ListRooms)
            .into_actor(self)
            .then(|res, _, ctx| {
                if let Ok(rooms) = res {
                    for room in rooms {
                        ctx.text(room);
                    }
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    // 向当前房间发送消息。
    pub fn send_msg(&self, msg: &str) {
        let content = format!(
            "{}: {msg}",
            self.name.clone().unwrap_or_else(|| "anon".to_owned()),
        );

        let msg = SendMessage(self.room.clone(), self.id, content);
        self.issue_system_async(msg);
    }
}

// 为 `WsChatSession` 实现 `Actor` trait。
impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    // 在会话启动时加入 "main" 房间。
    fn started(&mut self, ctx: &mut Self::Context) {
        self.join_room("main", ctx);
    }

    // 在会话停止时记录消息。
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!(
            "WsChatSession closed for {}({}) in room {}",
            self.name.clone().unwrap_or_else(|| "anon".to_owned()),
            self.id,
            self.room
        );
    }
}

// 为 `ChatMessage` 消息实现 `Handler` trait。
impl Handler<ChatMessage> for WsChatSession {
    type Result = ();

    // 处理 `ChatMessage` 消息，将消息发送给客户端。
    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

// 为 WebSocket 消息实现 `StreamHandler` trait。
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    // 处理传入的 WebSocket 消息。
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        log::debug!("WEBSOCKET MESSAGE: {msg:?}");

        match msg {
            ws::Message::Text(text) => {
                let msg = text.trim();

                // 处理以 '/' 开头的命令。
                if msg.starts_with('/') {
                    let mut command = msg.splitn(2, ' ');

                    match command.next() {
                        Some("/list") => self.list_rooms(ctx),

                        Some("/join") => {
                            if let Some(room_name) = command.next() {
                                self.join_room(room_name, ctx);
                            } else {
                                ctx.text("!!! room name is required");
                            }
                        }

                        Some("/name") => {
                            if let Some(name) = command.next() {
                                self.name = Some(name.to_owned());
                                ctx.text(format!("name changed to: {name}"));
                            } else {
                                ctx.text("!!! name is required");
                            }
                        }

                        _ => ctx.text(format!("!!! unknown command: {msg:?}")),
                    }

                    return;
                }
                self.send_msg(msg);
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}