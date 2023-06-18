//importing necessary modules and dependencies.
use std::io::Write;
use std::sync::{Arc, Mutex};
use crate::message_handler::MessageHandler;

/**
 * The code defines a struct ChatServer that holds the shared data for the chat server. 
 * messages and clients are filelds that are wrapped in Arc<Mutex<>> to provide thread-safe access.
 */
pub struct ChatServer {
    pub messages: Arc<Mutex<Vec<String>>>,
    pub clients: Arc<Mutex<Vec<(std::net::TcpStream, String)>>>,
}

/**
 * The ChatServer struct implements the MessageHandler trait, which defines the methods handle_message and send_message. 
 * This allows the ChatServer to handle incoming messages and send messages to all clients.
 */
impl MessageHandler for ChatServer {
    /**
     * The messages vector is locked using a MutexGuard to ensure exclusive access. 
     * The message, along with the username, is added to the messages vector.
     */
    fn handle_message(&self, username: &str, message: &str) {
        // Lock the messages vector to access and modify it
        let mut messages: std::sync::MutexGuard<'_, Vec<String>> = self.messages.lock().unwrap();
        messages.push(format!("{}: {}", username, message));
        // Call the send_message method to send the message to all clients
        self.send_message(username, message);
        // Display the received message to the console
        println!("Received message from {}: {}", username, message);
    }

    /**
     * Send the message to all connected clients. 
     * The clients vector is locked, and then the formatted message is sent to each client.
     */
    fn send_message(&self, username: &str, message: &str) {
        // Lock the clients vector to access and modify it
        let mut clients: std::sync::MutexGuard<'_, Vec<(std::net::TcpStream, String)>> = self.clients.lock().unwrap();
        // Format the message with the username
        let formatted_message = format!("{}: {}", username, message);
        // Iterate over each client and send the message
        for (client, _) in clients.iter_mut() {
            // The formatted message is converted to bytes and written to the client's TCP stream using write_all
            client.write_all(formatted_message.as_bytes()).unwrap();
            // A newline character (\n) is sent to separate messages.
            client.write_all(b"\n").unwrap();
            // A flush is called to ensure the message is sent immediately.
            client.flush().unwrap();
        }
    }
}
