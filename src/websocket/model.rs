use tokio::sync::{mpsc, oneshot};

/// Connection ID.
pub type ConnId = uuid::Uuid;

/// Room ID.
pub type RoomId = String;

/// Message sent to a room/client.
pub type Msg = String;

/// A command received by the [`ChatServer`].
#[derive(Debug)]
pub enum Command {
    Connect {
        conn_tx: mpsc::UnboundedSender<Msg>,
        res_tx: oneshot::Sender<ConnId>,
    },

    Disconnect {
        conn: ConnId,
    },

    List {
        res_tx: oneshot::Sender<Vec<RoomId>>,
    },

    Join {
        conn: ConnId,
        room: RoomId,
        res_tx: oneshot::Sender<()>,
    },

    Message {
        msg: Msg,
        conn: ConnId,
        res_tx: oneshot::Sender<()>,
    },
}
