pub struct Message {
    /// type of this message
    message_type: String,
    /// the room with which this message is associated
    room_id: String,
    /// the content of this message
    content: String,
    /// the username of the sender of this message
    sender_username: String,
    /// the id of the sender of this message
    sender_id: i32,
    /// the username of the recipient of this message
    recipient_username: String,
    /// the id of the recipient of this message
    recipient_id: i32,
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
