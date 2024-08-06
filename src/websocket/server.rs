use std::collections::HashMap;

use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use uuid::Uuid;

use crate::websocket::model::{ChatMessage, JoinRoom, LeaveRoom, ListRooms, SendMessage};

type Client = Recipient<ChatMessage>;
type Room = HashMap<Uuid, Client>;

#[derive(Default)]
pub struct WsChatServer {
    rooms: HashMap<String, Room>,
}

// WsChatServer实现
impl WsChatServer {
    // 从self.rooms中获取一个房间，然后将该房间从self.rooms中移除并返回它。
    fn take_room(&mut self, room_name: &str) -> Option<Room> {
        let room = self.rooms.get_mut(room_name)?;
        let room = std::mem::take(room);
        Some(room)
    }

    // 将一个客户端添加到一个房间中。如果房间不存在，则创建一个新房间。
    fn add_client_to_room(&mut self, room_name: &str, id: Option<Uuid>, client: Client) -> Uuid {
        let mut id = id.unwrap_or_else(|| Uuid::new_v4());

        if let Some(room) = self.rooms.get_mut(room_name) {
            loop {
                if room.contains_key(&id) {
                    id = Uuid::new_v4();
                } else {
                    break;
                }
            }

            room.insert(id, client);
            return id;
        }

        // Create a new room for the first client
        let mut room: Room = HashMap::new();

        room.insert(id, client);
        self.rooms.insert(room_name.to_owned(), room);

        id
    }

    /**
     * 从self.rooms中获取一个房间，然后将该房间中的每个客户端都发送一个消息。
     * 如果发送成功，则将客户端添加回房间中。
     */
    fn send_chat_message(&mut self, room_name: &str, msg: &str, _src: Uuid) -> Option<()> {
        let mut room = self.take_room(room_name)?;

        for (id, client) in room.drain() {
            if client.try_send(ChatMessage(msg.to_owned())).is_ok() {
                self.add_client_to_room(room_name, Some(id), client);
            }
        }

        Some(())
    }
}

// 为WsChatServer实现Actor
impl Actor for WsChatServer {
    type Context = Context<Self>;

    /**
     * 在Actor启动时，我们订阅了LeaveRoom和SendMessage消息。
     * 这样，我们就可以在Actor中处理这些消息。
     */
    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<LeaveRoom>(ctx);
        self.subscribe_system_async::<SendMessage>(ctx);
    }
}

/**
 * 为JoinRoom消息实现Handler trait。
 * 这个Handler trait的Result类型是MessageResult<JoinRoom>。
 * 这意味着我们将返回一个MessageResult，其中包含一个Uuid。
 */
impl Handler<JoinRoom> for WsChatServer {
    type Result = MessageResult<JoinRoom>;

    // 当处理JoinRoom消息时，我们将客户端添加到房间中，并返回一个MessageResult。
    fn handle(&mut self, msg: JoinRoom, _ctx: &mut Self::Context) -> Self::Result {
        let JoinRoom(room_name, client_name, client) = msg;

        let id = self.add_client_to_room(&room_name, None, client);
        let join_msg = format!(
            "{} joined {room_name}",
            client_name.unwrap_or_else(|| "anon".to_owned()),
        );

        self.send_chat_message(&room_name, &join_msg, id);
        MessageResult(id)
    }
}

// 为LeaveRoom消息实现Handler
impl Handler<LeaveRoom> for WsChatServer {
    type Result = ();

    fn handle(&mut self, msg: LeaveRoom, _ctx: &mut Self::Context) {
        if let Some(room) = self.rooms.get_mut(&msg.0) {
            room.remove(&msg.1);
        }
    }
}

// 为ListRooms消息实现Handler
impl Handler<ListRooms> for WsChatServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.rooms.keys().cloned().collect())
    }
}

// 为SendMessage消息实现Handler
impl Handler<SendMessage> for WsChatServer {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, _ctx: &mut Self::Context) {
        let SendMessage(room_name, id, msg) = msg;
        self.send_chat_message(&room_name, &msg, id);
    }
}

impl SystemService for WsChatServer {}
impl Supervised for WsChatServer {}
