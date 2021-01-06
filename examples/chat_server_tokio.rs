use tokio::net::{TcpListener, TcpStream};
use std::io::{Read};
use tokio::io::AsyncWriteExt;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(50)).await;
        let mut stream: TcpStream = TcpStream::connect("127.0.0.1:3333").await.unwrap();
        let _ = stream.write(b"Hello world!").await;
    });

    let mut data = [0 as u8; 50];
    let listener = TcpListener::bind("127.0.0.1:3333").await?;
    let (tokio_tcp_stream, _) = listener.accept().await?;
    let mut std_tcp_stream = tokio_tcp_stream.into_std()?;
    tokio::time::sleep(Duration::from_millis(100)).await;
    let size = std_tcp_stream.read(&mut data)?;
    assert_eq!("Hello world!", std::str::from_utf8(&data[0..size])?);
    Ok(())
}