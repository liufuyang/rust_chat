use tokio::net::TcpStream;
use tokio::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:3333").await?;
    let mut data = [0 as u8; 50]; // using 50 byte buffer

    // Write some data.
    stream.write_all(b"hello world!\n").await?;
    stream.read(&mut data).await?;
    println!("data read: [{}]", std::str::from_utf8(&data).unwrap());
    // Note, without reading from server and exit here will cause a "Connection reset by peer" error on server while server's trying to read again

    Ok(())
}