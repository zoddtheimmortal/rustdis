use bytes::BytesMut;
use rustdis::{helper::buffer_to_array, Command, Db};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let mut db = Db::new();
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("connection accepted:  {:?}", socket);

        let mut buff = BytesMut::with_capacity(1024);
        socket.read_buf(&mut buff).await?;

        let attr = buffer_to_array(&mut buff);
        let command = Command::get_command(&attr[0]);

        process_query(command, attr, &mut socket, &mut db).await?;
    }
    // Ok(())
}

async fn process_query(
    command: Command,
    attrs: Vec<String>,
    socket: &mut TcpStream,
    db: &mut Db,
) -> std::io::Result<()> {
    match command {
        Command::Get => Ok(()),
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
