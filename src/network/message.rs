use serde::{Serialize, Deserialize};

/// Defines the types of messages that can be sent in the network.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    NewBlock,
    NewTransaction,
    RequestFullChain,
    ResponseFullChain,
    // Add other message types as necessary for your protocol
}

/// Represents a message to be sent over the network.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub message_type: MessageType,
    pub content: String, // This could be a serialized structure depending on your needs.
}

impl Message {
    /// Creates a new message with the given type and content.
    pub fn new(message_type: MessageType, content: String) -> Self {
        Message {
            message_type,
            content,
        }
    }

    /// Serializes the message for transmission.
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize message")
    }

    /// Deserializes a message from a string.
    pub fn deserialize(input: &str) -> serde_json::Result<Self> {
        serde_json::from_str(input)
    }
}
use serde::{Serialize, Deserialize};

/// Defines the types of messages that can be sent in the network.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    NewBlock,
    NewTransaction,
    RequestFullChain,
    ResponseFullChain,
    // Add other message types as necessary for your protocol
}

/// Represents a message to be sent over the network.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub message_type: MessageType,
    pub content: String, // This could be a serialized structure depending on your needs.
}

impl Message {
    /// Creates a new message with the given type and content.
    pub fn new(message_type: MessageType, content: String) -> Self {
        Message {
            message_type,
            content,
        }
    }

    /// Serializes the message for transmission.
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize message")
    }

    /// Deserializes a message from a string.
    pub fn deserialize(input: &str) -> serde_json::Result<Self> {
        serde_json::from_str(input)
    }
}
