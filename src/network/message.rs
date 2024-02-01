use serde::{Serialize, Deserialize};

/// Enumerates the types of messages that can be sent in the network.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    NewBlock,
    NewTransaction,
    RequestFullChain,
    ResponseFullChain,
    // Extend with additional message types as your protocol evolves.
}

/// Represents a network message with a specific type and content.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub message_type: MessageType,
    // Consider using a more structured content type if necessary.
    pub content: String, // Serialized JSON string for flexibility.
}

impl Message {
    /// Constructs a new message with the specified type and content.
    pub fn new(message_type: MessageType, content: String) -> Self {
        Message { message_type, content }
    }

    /// Serializes the message to a JSON string for transmission.
    /// Returns a `Result` to handle potential serialization errors gracefully.
    pub fn serialize(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    /// Deserializes a message from a JSON string.
    /// This static method returns a `Result`, making error handling explicit.
    pub fn deserialize(input: &str) -> serde_json::Result<Self> {
        serde_json::from_str(input)
    }
}
use serde::{Serialize, Deserialize};

/// Enumerates the types of messages that can be sent in the network.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    NewBlock,
    NewTransaction,
    RequestFullChain,
    ResponseFullChain,
    // Extend with additional message types as your protocol evolves.
}

/// Represents a network message with a specific type and content.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub message_type: MessageType,
    // Consider using a more structured content type if necessary.
    pub content: String, // Serialized JSON string for flexibility.
}

impl Message {
    /// Constructs a new message with the specified type and content.
    pub fn new(message_type: MessageType, content: String) -> Self {
        Message { message_type, content }
    }

    /// Serializes the message to a JSON string for transmission.
    /// Returns a `Result` to handle potential serialization errors gracefully.
    pub fn serialize(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    /// Deserializes a message from a JSON string.
    /// This static method returns a `Result`, making error handling explicit.
    pub fn deserialize(input: &str) -> serde_json::Result<Self> {
        serde_json::from_str(input)
    }
}
