use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(
    mut stream: TcpStream,
    sender: Sender<(SocketAddr, String)>,
    receiver: Receiver<String>,
    peer_addr: SocketAddr,
    sender_map: Arc<Mutex<HashMap<SocketAddr, Sender<String>>>>,
) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer

    let mut stream_clone = stream.try_clone().expect("clone failed...");
    std::thread::spawn(move || {
        while match receiver.recv() {
            Ok(msg) => {
                stream_clone.write(msg.as_bytes()).unwrap();
                true
            }
            Err(e) => {
                println!("Client {} disconnected. Message: {}", peer_addr, e);
                false
            }
        } {}
    });

    while match stream.read(&mut data) {
        Ok(0) => {
            // Stream ended as client closed, clean up stuff here
            sender_map.lock().unwrap().remove(&peer_addr);
            false
        }
        Ok(size) => {
            // echo everything!
            // stream.write(&data[0..size]).unwrap();
            let to_send = std::str::from_utf8(&data[0..size]).unwrap_or("?").into();
            match sender.send((peer_addr.clone(), to_send)) {
                Ok(_) => true,
                Err(e) => {
                    println!("{}", e);
                    sender_map.lock().unwrap().remove(&peer_addr);
                    false
                }
            }
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");

    let (sender, recv_agent) = channel::<(SocketAddr, String)>();
    let sender_map: Arc<Mutex<HashMap<SocketAddr, Sender<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let sender_map_copy = sender_map.clone();

    // Agent thread
    thread::spawn(move || {
        while match recv_agent.recv() {
            Ok((addr, msg)) => {
                print!("[{}]: {}", &addr, msg.as_str());
                for (key, sender_in_map) in sender_map_copy.lock().unwrap().iter() {
                    if key == &addr {
                        continue;
                    }

                    match sender_in_map.send(format!("[{}]: {}", addr, msg)) {
                        Ok(_) => {}
                        Err(e) => println!("{}", e),
                    }
                }
                true
            }
            Err(e) => {
                println!("{:?}", e);
                true
            }
        } {}
    });

    for stream in listener.incoming() {
        let sender_copy = sender.clone();
        let (sender_in_map, recv) = channel();
        match stream {
            Ok(mut stream) => {
                let peer_addr = stream.peer_addr().unwrap();
                println!("New connection: {}", peer_addr);
                stream
                    .write(format!("Welcome {}\n", peer_addr).as_bytes())
                    .unwrap();

                let sender_map_copy = sender_map.clone();
                sender_map_copy
                    .lock()
                    .unwrap()
                    .insert(peer_addr.clone(), sender_in_map.clone());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream, sender_copy, recv, peer_addr, sender_map_copy)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
