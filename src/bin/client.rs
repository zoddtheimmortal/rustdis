use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8081").await?;

    // command format -> <cmd name> <key> <value>
    stream.write_all(b"set key value").await?;

    Ok(())
}
