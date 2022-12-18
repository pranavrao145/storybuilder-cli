use std::io;
use std::io::Write;
use std::time::Duration;

use crossterm::event::{poll, Event, KeyCode};

use crate::cli::Cli;
use crate::utils::{get_all_story_lines, get_players_list};

pub async fn update_game_waiting_screen_ui(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();

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

    let players = get_players_list(cli).await?;

    writeln!(&mut stdout.lock(), "\nCurrent players ({}):", players.len())?;

    for player in &players {
        writeln!(&mut stdout.lock(), "{}", player)?;
    }

    writeln!(&mut stdout.lock(), "\n")?;

    let message_str: String;

    if *cli.current_player_info.is_host && players.len() > 1 {
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
    clearscreen::clear().unwrap();

    let stdout = io::stdout();
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

pub async fn update_end_game_ui(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    clearscreen::clear().unwrap();

    let stdout = io::stdout();
    writeln!(
        &mut stdout.lock(),
        "Here is the story you constructed this round:\n"
    )?;

    let story_lines = get_all_story_lines(cli).await?;

    for story_line in story_lines {
        println!("{}", story_line);
    }

    println!("\nPress Enter to quit.");

    loop {
        if poll(Duration::from_millis(500)).unwrap() {
            let event = crossterm::event::read().unwrap();

            if event == Event::Key(KeyCode::Enter.into()) {
                break;
            }
        }
    }

    clearscreen::clear().unwrap();

    Ok(())
}
