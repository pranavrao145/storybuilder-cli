use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    /// type of this message
    #[serde(rename(deserialize = "messageType", serialize = "messageType"))]
    pub message_type: String,
    /// the room with which this message is associated
    #[serde(rename(deserialize = "roomId", serialize = "roomId"))]
    pub room_id: String,
    /// the content of this message
    pub content: String,
    /// the username of the sender of this message
    #[serde(rename(deserialize = "senderUsername", serialize = "senderUsername"))]
    pub sender_username: String,
    /// the id of the sender of this message
    #[serde(rename(deserialize = "senderId", serialize = "senderId"))]
    pub sender_id: i32,
    /// the username of the recipient of this message
    #[serde(rename(deserialize = "recipientUsername", serialize = "recipientUsername"))]
    pub recipient_username: String,
    /// the id of the recipient of this message
    #[serde(rename(deserialize = "recipientId", serialize = "recipientId"))]
    pub recipient_id: i32,
}

impl Message {
    pub fn new() -> Self {
        Self {
            message_type: "".to_string(),
            room_id: "".to_string(),
            content: "".to_string(),
            sender_username: "".to_string(),
            sender_id: -1,
            recipient_username: "".to_string(),
            recipient_id: -1,
        }
    }
}
