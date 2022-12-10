use futures_util::{SinkExt, TryStreamExt};
use std::io::Write;
use std::{error::Error, io, time::Duration};

use crossterm::event::{poll, Event, KeyCode};
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;

use crate::{cli::Cli, message::Message, ui_utils::update_game_waiting_screen_ui};

pub async fn run_game(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    wait_for_game_start(cli).await?;
    // play_game(&cli).await?;
    end_game().await?;

    Ok(())
}

pub async fn end_game() -> Result<(), Box<dyn std::error::Error>> {
    println!("The game has ended.");
    Ok(())
}

pub async fn wait_for_game_start(cli: &Cli) -> Result<(), Box<dyn Error>> {
    let mut server_url = cli.server_url.clone();

    server_url
        .set_scheme("ws")
        .expect("Failed to set server url scheme to ws.");
    server_url.set_path("ws");
    server_url.set_query(Some(
        format!(
            "username={}&roomId={}&isHost={}&clientId={}",
            cli.current_player_info.username,
            cli.current_player_info.room_id,
            cli.current_player_info.is_host,
            cli.current_player_info.client_id
        )
        .as_str(),
    ));

    // listen for enter key if player is host
    let (socket, _) = connect_async(server_url).await?;

    let (mut write, mut read) = socket.split();

    if *cli.current_player_info.is_host {
        let cli_clone = cli.clone();

        tokio::spawn(async move {
            loop {
                if poll(Duration::from_millis(1_000)).unwrap() {
                    let event = crossterm::event::read().unwrap();

                    if event == Event::Key(KeyCode::Enter.into()) {
                        // write join message to socket

                        let msg: Message = Message {
                            message_type: "start".to_string(),
                            room_id: *cli_clone.current_player_info.room_id,
                            content: "".to_string(),
                            sender_username: *cli_clone.current_player_info.username,
                            sender_id: *cli_clone.current_player_info.client_id,
                            recipient_username: "".to_string(),
                            recipient_id: -1,
                        };

                        let msg_stringified = serde_json::to_string(&msg)
                            .expect("Failed to convert struct to jsonified string.");

                        write
                            .send(tokio_tungstenite::tungstenite::Message::Text(
                                msg_stringified,
                            ))
                            .await
                            .expect("Failed to send join message to server.");

                        write.close().await.unwrap();

                        return;
                    }
                }
            }
        });
    }

    let cli_clone = cli.clone();

    tokio::spawn(async move {
        loop {
            let data = read.try_next().await;

            match data {
                Ok(Some(raw_message)) => {
                    let message: Message = serde_json::from_str(&raw_message.into_text().unwrap())
                        .unwrap_or_else(|_| Message::new());

                    match message.message_type.as_str() {
                        "join" | "leave" => {
                            update_game_waiting_screen_ui(&cli_clone).await.unwrap();
                        }
                        "start" => {
                            return;
                        }
                        _ => {}
                    }
                }
                Ok(None) => {}
                Err(_) => {
                    let stdout = io::stdout();
                    writeln!(
                        &mut stdout.lock(),
                        "Fatal error: lost connection to server."
                    )
                    .unwrap();

                    return;
                }
            }
        }
    })
    .await?;

    Ok(())
}
