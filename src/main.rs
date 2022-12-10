use dotenv::dotenv;
use std::error::Error;

use storybuilder_cli::{
    cli::Cli,
    player_info::PlayerInfo,
    utils::{
        get_game_type, get_generated_client_id, get_generated_game_code, get_join_code,
        get_server_url, get_username, join_code_exists,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load environment variables
    dotenv().ok();

    let server_url = get_server_url().await?;
    let mut cli = Cli::new(PlayerInfo::new(), server_url);
    let game_type = get_game_type().await?;

    if game_type == "New Game" {
        cli.current_player_info.is_host = Box::new(true);
        cli.current_player_info.room_id = Box::new(get_generated_game_code(&cli).await?);
        cli.current_player_info.username = Box::new(get_username().await?);
        cli.current_player_info.client_id = Box::new(get_generated_client_id(&cli).await?);
    } else if game_type == "Join Game" {
        cli.current_player_info.is_host = Box::new(false);

        let potential_join_code = get_join_code().await?;

        if !join_code_exists(&cli, &potential_join_code).await? {
            clearscreen::clear().unwrap();
            println!("Invalid join code provided. Quitting...");
            return Ok(());
        }

        cli.current_player_info.room_id = Box::new(potential_join_code);
        cli.current_player_info.username = Box::new(get_username().await?);
        cli.current_player_info.client_id = Box::new(get_generated_client_id(&cli).await?);
    } else {
        println!("Quitting...");

        return Ok(());
    }

    Ok(())
}
