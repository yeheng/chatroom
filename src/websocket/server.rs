use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::AtomicUsize,
        Arc,
    },
};
use tokio::io;
use uuid::Uuid;

use crate::websocket::model::{Command, ConnId, Msg, RoomId};
use tokio::sync::{mpsc, oneshot, RwLock};

// 定义默认房间名称为常量
const DEFAULT_ROOM_NAME: &str = "main";

#[derive(Debug)]
pub struct ChatServer {
    /// Map of connection IDs to their message receivers.
    sessions: RwLock<HashMap<ConnId, mpsc::UnboundedSender<Msg>>>,

    /// Map of room name to participant IDs in that room.
    rooms: RwLock<HashMap<RoomId, HashSet<ConnId>>>,

    /// Tracks total number of historical connections established.
    visitor_count: Arc<AtomicUsize>,

    /// Command receiver.
    cmd_rx: mpsc::UnboundedReceiver<Command>,
}

impl ChatServer {
    pub fn new() -> (Self, ChatServerHandle) {
        let rooms = HashMap::from([(DEFAULT_ROOM_NAME.to_owned(), HashSet::new())]);
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        (
            Self {
                sessions: RwLock::new(HashMap::new()),
                rooms: RwLock::new(rooms),
                visitor_count: Arc::new(AtomicUsize::new(0)),
                cmd_rx,
            },
            ChatServerHandle { cmd_tx },
        )
    }

    pub async fn send_system_message(&self, room: &str, skip: ConnId, msg: impl Into<Msg>) {
        if let Some(participants) = self.rooms.read().await.get(room) {
            let msg = msg.into();
            let sessions_lock = self.sessions.write().await;

            for conn_id in participants {
                if *conn_id != skip {
                    match sessions_lock.get(conn_id) {
                        None => {
                            log::warn!("Failed to find session for connection: {}", conn_id);
                        }
                        Some(tx) => {
                            if let Err(e) = tx.send(msg.clone()) {
                                log::error!("Failed to send message: {:?}", e);
                            }
                        }
                    }
                }
            }
        }
    }

    async fn send_message(&self, conn: ConnId, msg: impl Into<Msg>) {
        if let Some(room) = self.rooms.read().await.iter()
            .find_map(|(room, participants)| participants.contains(&conn).then_some(room)) {
            self.send_system_message(room, conn, msg).await;
        }
    }

    async fn connect(&mut self, tx: mpsc::UnboundedSender<Msg>) -> ConnId {
        log::info!("Someone joined");
        let id = Uuid::new_v4();
        self.sessions.write().await.insert(id, tx);
        self.rooms.write().await.entry(DEFAULT_ROOM_NAME.to_owned()).or_default().insert(id);

        self.visitor_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        id
    }

    async fn disconnect(&mut self, conn_id: ConnId) {
        log::info!("Someone disconnected");

        let rooms = self.copy_write_rooms(conn_id)
            .await;

        self.visitor_count.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);

        for room in rooms {
            self.send_system_message(&room, conn_id, "Someone disconnected").await;
        }
    }

    async fn list_rooms(&self) -> Vec<RoomId> {
        self.rooms.read().await.keys().cloned().collect()
    }

    async fn copy_write_rooms(&mut self, conn_id: ConnId) -> Vec<RoomId> {
        let mut rooms = Vec::new();
        for (n, participants) in self.rooms.write().await.iter_mut() {
            if participants.remove(&conn_id) {
                rooms.push(n.to_owned());
            }
        }
        rooms
    }

    async fn join_room(&mut self, conn_id: ConnId, room: RoomId) {
        let rooms = self.copy_write_rooms(conn_id).await;

        for room in rooms {
            self.send_system_message(&room, conn_id, "Someone disconnected").await;
        }

        self.rooms.write().await.entry(room.clone()).or_default().insert(conn_id);
        self.send_system_message(&room, conn_id, "Someone connected").await;
    }

    pub async fn run(mut self) -> io::Result<()> {
        while let Some(cmd) = self.cmd_rx.recv().await {
            match cmd {
                Command::Connect { conn_tx, res_tx } => {
                    let conn_id = self.connect(conn_tx).await;
                    let _ = res_tx.send(conn_id);
                }

                Command::Disconnect { conn } => {
                    self.disconnect(conn).await;
                }

                Command::List { res_tx } => {
                    let _ = res_tx.send(self.list_rooms().await);
                }

                Command::Join { conn, room, res_tx } => {
                    self.join_room(conn, room).await;
                    let _ = res_tx.send(());
                }

                Command::Message { conn, msg, res_tx } => {
                    self.send_message(conn, msg).await;
                    let _ = res_tx.send(());
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ChatServerHandle {
    cmd_tx: mpsc::UnboundedSender<Command>,
}

impl ChatServerHandle {
    pub async fn connect(&self, conn_tx: mpsc::UnboundedSender<Msg>) -> ConnId {
        let (res_tx, res_rx) = oneshot::channel();
        self.cmd_tx.send(Command::Connect { conn_tx, res_tx }).unwrap();
        res_rx.await.unwrap()
    }

    pub async fn list_rooms(&self) -> Vec<RoomId> {
        let (res_tx, res_rx) = oneshot::channel();
        self.cmd_tx.send(Command::List { res_tx }).unwrap();
        res_rx.await.unwrap()
    }

    pub async fn join_room(&self, conn: ConnId, room: impl Into<RoomId>) {
        let (res_tx, res_rx) = oneshot::channel();
        self.cmd_tx.send(Command::Join {
            conn,
            room: room.into(),
            res_tx,
        }).unwrap();
        res_rx.await.unwrap();
    }

    pub async fn send_message(&self, conn: ConnId, msg: impl Into<Msg>) {
        let (res_tx, res_rx) = oneshot::channel();
        self.cmd_tx.send(Command::Message {
            msg: msg.into(),
            conn,
            res_tx,
        }).unwrap();
        res_rx.await.unwrap();
    }

    pub fn disconnect(&self, conn: ConnId) {
        self.cmd_tx.send(Command::Disconnect { conn }).unwrap();
    }
}
