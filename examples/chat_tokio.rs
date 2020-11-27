use tokio::net::TcpStream;
use tokio::prelude::*;
use std::error::Error;
use std::net::Shutdown;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:3333").await?;

    // Write some data.
    stream.write_all(b"hello world!\n").await?;
    // stream.shutdown(Shutdown::Both).expect("Shutdown error");
    Ok(())
}