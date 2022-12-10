#[derive(Clone)]
pub struct PlayerInfo {
    /// the username of this player
    pub username: Box<String>,
    /// the id of the room this player is in
    pub room_id: Box<String>,
    /// whether or not this player is the host of their game
    pub is_host: Box<bool>,
    /// the clietn id of this player
    pub client_id: Box<i32>,
}

impl PlayerInfo {
    pub fn new() -> Self {
        Self {
            username: Box::new("".to_string()),
            room_id: Box::new("".to_string()),
            is_host: Box::new(false),
            client_id: Box::new(-1),
        }
    }
}
