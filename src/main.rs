mod chat_server;
mod message_handler;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use chat_server::ChatServer;
use message_handler::MessageHandler;

fn handle_client(mut stream: TcpStream, server: Arc<dyn MessageHandler>) {
    let mut buffer: [u8; 1024] = [0; 1024];

    // Read the username from the client
    stream.write_all(b"Welcome! Please enter your username: ").unwrap();
    stream.flush().unwrap();
    // stream.read(&mut buffer).unwrap();
    let bytes_read: usize = stream.read(&mut buffer).unwrap();
    let username: String = String::from_utf8_lossy(&buffer[..bytes_read])
            .trim_end_matches(|c: char| c == '\n' || c == '\r')
            .to_string();

    println!("{} joined the chat\n", username);

    loop {
        match stream.read(&mut buffer) {
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                }
                let message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..bytes]);
                server.handle_message(&username, &message); // Pass the username to handle_message
            }
            Err(_) => {
                break;
            }
        }
    }
}

fn main() {
    let messages: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let clients: Arc<Mutex<Vec<(TcpStream, String)>>> = Arc::new(Mutex::new(Vec::new()));
    let server: Arc<ChatServer> = Arc::new(ChatServer {
        messages: Arc::clone(&messages),
        clients: Arc::clone(&clients),
    });

    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind address");
    println!("Chat server started on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let server: Arc<ChatServer> = Arc::clone(&server);
                let mut clients: std::sync::MutexGuard<'_, Vec<(TcpStream, String)>> = clients.lock().unwrap();
                clients.push((stream.try_clone().unwrap(), String::new()));
                thread::spawn(move || {
                    handle_client(stream, server);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
