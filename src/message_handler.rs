
pub trait MessageHandler {
    fn handle_message(&self, username: &str, message: &str);
    fn send_message(&self, username: &str, message: &str);
}
