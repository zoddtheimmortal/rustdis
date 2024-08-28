use bytes::BytesMut;
use rustdis::{
    helper::buffer_to_array,
    listener::{self, Listener},
    Command, Db,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    signal,
    sync::{broadcast, mpsc},
};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    let shutdown = signal::ctrl_c();
    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);

    let mut listener = Listener::new(
        listener,
        notify_shutdown,
        shutdown_complete_tx,
        shutdown_complete_rx,
    );

    tokio::select! {
        res = server::run(&mut listener) => {
            if let Err(_err)=res{
                println!("Failed to accept connection")
            }
        }
        _ = shutdown =>{
            println!("Inside shutdown loop")
        }
    }
    Ok(())
}

async fn process_query(
    command: Command,
    attrs: Vec<String>,
    socket: &mut TcpStream,
    db: &mut Db,
) -> std::io::Result<()> {
    match command {
        Command::Get => {
            let result = db.read(&attrs);
            match result {
                Ok(result) => {
                    socket.write_all(&result).await?;
                }
                Err(_err) => {
                    println!("no key found: {}", _err);
                    let _ = socket.write_all(b"").await;
                }
            }
            Ok(())
        }
        Command::Set => {
            let resp = db.write(&attrs);

            match resp {
                Ok(result) => {
                    println!("set result: {}", result);
                    socket.write_all(&result.as_bytes()).await?;
                }
                Err(_err) => {
                    socket.write_all(b"Error").await?;
                }
            }

            Ok(())
        }
        Command::Invalid => Ok(()),
    }
}
