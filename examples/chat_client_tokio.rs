use tokio::net::TcpStream;
use std::error::Error;
use std::io::{Read, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let stream = TcpStream::connect("127.0.0.1:3333").await?;
    let mut stream = stream.into_std().unwrap();
    let mut data = [0 as u8; 50]; // using 50 byte buffer

    // Write some data.
    let _ = stream.write(b"hello world!\n");
    let _ = stream.read(&mut data);
    println!("data read: [{}]", std::str::from_utf8(&data).unwrap());
    // Note, without reading from server and exit here will cause a "Connection reset by peer" error on server while server's trying to read again

    Ok(())
}