use tokio::{
    net::TcpStream,
    sync::{broadcast, mpsc},
};

use crate::Db;

pub struct Handler {
    pub connection: Connection,
    pub db: Db,
    pub shutdown: Shutdown,

    _shutdown_complete: mpsc::Sender<()>,
}

pub struct Connection {
    pub stream: TcpStream,
}

pub struct Shutdown {
    shutdown: bool,
    notify: broadcast::Receiver<()>,
}
