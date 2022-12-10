use url::Url;

use crate::player_info::PlayerInfo;

pub struct Cli {
    /// the player info for this instance of the CLI
    pub current_player_info: PlayerInfo,
    /// the server url associated with this instance of the CLI
    pub server_url: Url,
}

impl Cli {
    pub fn new(current_player_info: PlayerInfo, server_url: Url) -> Self {
        Self {
            current_player_info,
            server_url,
        }
    }
}
