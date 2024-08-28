use tokio::{
    net::TcpListener,
    sync::{broadcast, mpsc},
};

use crate::Db;

pub struct Listener {
    pub db: Db,
    pub listener: TcpListener,
    pub notify_shutdown: broadcast::Sender<()>,
    pub shutdown_complete_rx: mpsc::Receiver<()>,
    pub shutdown_complete_tx: mpsc::Sender<()>,
}
