//importing necessary modules and dependencies.
mod chat_server;
mod message_handler;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use chat_server::ChatServer;
use message_handler::MessageHandler;

/**
 * Responsible for handling client connections. 
 */
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

    // Display Message
    println!("{} joined the chat\n", username);

    // Loop to read and handle incoming messages from the client.
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

/**
 * Entry point of the program
 */
fn main() {
    // Create shared data structures using Arc and Mutex
    let messages: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let clients: Arc<Mutex<Vec<(TcpStream, String)>>> = Arc::new(Mutex::new(Vec::new()));
    let server: Arc<ChatServer> = Arc::new(ChatServer {
        messages: Arc::clone(&messages),
        clients: Arc::clone(&clients),
    });

    // Bind the TCP listener to the address 127.0.0.1:8080 and starts listening for incoming connections.
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind address");
    println!("Chat server started on 127.0.0.1:8080");

    // Accept incoming connections and handle each client in a separate thread
    // The client is added to the clients vector
    // The handle_client function is called in the new thread
    // Passing the client's TCP stream and the shared server object
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let server: Arc<ChatServer> = Arc::clone(&server);

                // Lock the clients vector to add the new client
                let mut clients: std::sync::MutexGuard<'_, Vec<(TcpStream, String)>> = clients.lock().unwrap();
                clients.push((stream.try_clone().unwrap(), String::new()));

                // Spawn a new thread to handle the client
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
