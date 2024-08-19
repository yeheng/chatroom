use actix::prelude::*;
use uuid::Uuid;

#[derive(Clone, Message)]
#[rtype(result = "()")]
// 聊天消息结构体，包含一个字符串消息
pub struct ChatMessage(pub String);

#[derive(Clone, Message)]
#[rtype(result = "uuid::Uuid")]
// 加入房间消息结构体，包含房间名、可选的用户名和消息接收者
pub struct JoinRoom(pub String, pub Option<String>, pub Recipient<ChatMessage>);

#[derive(Clone, Message)]
#[rtype(result = "()")]
// 离开房间消息结构体，包含房间名和用户ID
pub struct LeaveRoom(pub String, pub Uuid);

#[derive(Clone, Message)]
#[rtype(result = "Vec<String>")]
// 列出房间消息结构体
pub struct ListRooms;

#[derive(Clone, Message)]
#[rtype(result = "()")]
// 发送消息结构体，包含房间名、用户ID和消息内容
pub struct SendMessage(pub String, pub Uuid, pub String);