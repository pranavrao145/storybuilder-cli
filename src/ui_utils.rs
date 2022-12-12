use std::io::Write;
use std::{collections::HashMap, io};

use crate::cli::Cli;

pub async fn update_game_waiting_screen_ui(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut temp_server_url = cli.server_url.clone();

    temp_server_url.set_path("get_members");
    temp_server_url.set_query(Some(
        format!("roomId={}", cli.current_player_info.room_id).as_str(),
    ));

    let resp = reqwest::get(temp_server_url)
        .await?
        .json::<HashMap<String, Vec<String>>>()
        .await?;

    clearscreen::clear().unwrap();

    writeln!(
        &mut stdout.lock(),
        "Game code: {}",
        cli.current_player_info.room_id
    )?;

    writeln!(
        &mut stdout.lock(),
        "Username: {}",
        cli.current_player_info.username
    )?;

    let players = resp.get("roomMembers").unwrap();

    writeln!(&mut stdout.lock(), "\nCurrent players ({}):", players.len())?;

    for player in players {
        if *player == *cli.current_player_info.username {
            writeln!(&mut stdout.lock(), "{} (you)", player)?;
        } else {
            writeln!(&mut stdout.lock(), "{}", player)?;
        }
    }

    writeln!(&mut stdout.lock(), "\n")?;

    let message_str: String;

    if *cli.current_player_info.is_host {
        message_str = String::from("Waiting for players to join (press enter to start)...");
    } else {
        message_str = String::from("Waiting for players to join...");
    }

    writeln!(&mut stdout.lock(), "{}", message_str)?;

    Ok(())
}

pub async fn update_turn_waiting_screen_ui(
    current_player_username: Option<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    clearscreen::clear().unwrap();

    writeln!(&mut stdout.lock(), "Waiting for turn...")?;

    if current_player_username.is_some() {
        writeln!(
            &mut stdout.lock(),
            "Now playing: {}",
            current_player_username.unwrap()
        )?;
    }

    Ok(())
}
