// Declares a public trait (a collection of methods defined for an unknown type: self)
pub trait MessageHandler {
    // This method is responsible for handling a received message
    fn handle_message(&self, username: &str, message: &str);
    // This method is responsible for sending a message to one or more recipients
    fn send_message(&self, username: &str, message: &str);
}
