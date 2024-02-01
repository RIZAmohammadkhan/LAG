#[derive(Debug, Clone)]
pub enum MessageType {
    NewBlock,
    NewTransaction,
    // ... other message types
}

#[derive(Debug, Clone)]
pub struct Message {
    pub message_type: MessageType,
    pub content: String,  // This could be more complex depending on your needs
}

impl Message {
    pub fn new(message_type: MessageType, content: String) -> Self {
        Message { message_type, content }
    }
}
