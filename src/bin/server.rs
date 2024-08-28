use bytes::BytesMut;
use rustdis::{helper::buffer_to_array, Command};
use tokio::{io::AsyncReadExt, net::TcpListener};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Connection Accepted:  {:?}", socket);

        let mut buff = BytesMut::with_capacity(1024);
        socket.read_buf(&mut buff).await?;

        print!("Received: {:?}\n", buff);

        let attr = buffer_to_array(&mut buff);
        println!("Split array: {:?}", attr);

        let command = Command::get_command(&attr[0]);
    }
    // Ok(())
}
