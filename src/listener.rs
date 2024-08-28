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

impl Listener {
    pub fn new(
        listener: TcpListener,
        notify_shutdown: broadcast::Sender<()>,
        shutdown_complete_tx: mpsc::Sender<()>,
        shutdown_complete_rx: mpsc::Receiver<()>,
    ) -> Listener {
        Listener {
            listener,
            db: Db::new(),
            notify_shutdown,
            shutdown_complete_rx,
            shutdown_complete_tx: shutdown_complete_tx,
        }
    }
}
