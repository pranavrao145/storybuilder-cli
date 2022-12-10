use std::{
    collections::HashMap,
    env,
    error::Error,
    io::{self, Cursor},
};

use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};
use url::Url;

use crate::cli::Cli;

pub async fn get_server_url() -> Result<Url, Box<dyn Error>> {
    Ok(Url::parse(&env::var("STORYBUILDER_CLI_SERVER_URL")?)?)
}

pub async fn get_game_type() -> Result<String, Box<dyn Error>> {
    clearscreen::clear().unwrap();

    let options = SkimOptionsBuilder::default().height(Some("100%")).build()?;

    let input = "New Game\nJoin Game\nQuit";

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    let selected_item = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    let selected_item = selected_item.first().unwrap().output();

    Ok(selected_item.to_string())
}

pub async fn get_generated_game_code(cli: &Cli) -> Result<String, Box<dyn Error>> {
    let mut server_url = cli.server_url.clone();
    server_url.set_path("generate_room");

    let resp = reqwest::get(server_url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    let room_id = resp.get("roomId").unwrap();

    Ok(room_id.to_string())
}

pub async fn join_code_exists(cli: &Cli, join_code: &String) -> Result<bool, Box<dyn Error>> {
    let mut server_url = cli.server_url.clone();

    server_url.set_path("validate_room");
    server_url.set_query(Some(format!("roomId={}", join_code).as_str()));

    let resp = reqwest::get(server_url)
        .await?
        .json::<HashMap<String, bool>>()
        .await?;

    let exists = *resp.get("exists").unwrap();

    Ok(exists)
}

pub async fn get_generated_client_id(cli: &Cli) -> Result<i32, Box<dyn Error>> {
    let mut server_url = cli.server_url.clone();

    server_url.set_path("generate_client_id");
    server_url.set_query(Some(
        format!("roomId={}", cli.current_player_info.room_id).as_str(),
    ));

    let resp = reqwest::get(server_url)
        .await?
        .json::<HashMap<String, i32>>()
        .await?;

    let client_id = *resp.get("clientId").unwrap();

    Ok(client_id)
}

pub async fn get_username() -> Result<String, Box<dyn Error>> {
    print!("Please enter a username: ");

    io::Write::flush(&mut io::stdout())?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input)
}

pub async fn get_join_code() -> Result<String, Box<dyn Error>> {
    print!("Please enter a join code: ");

    io::Write::flush(&mut io::stdout())?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input)
}

pub async fn get_story_line(
    last_story_line: Option<&String>,
    last_player_username: Option<&String>,
) -> Result<String, Box<dyn Error>> {
    clearscreen::clear().unwrap();

    if last_story_line.is_some() && last_player_username.is_some() {
        println!(
            "Here is the last line of the story, given by {}: {}",
            last_player_username.unwrap(),
            last_story_line.unwrap()
        );
    }

    print!("Please enter the next line of the story: ");

    io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout.");

    let mut input = "".to_string();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get story line.");

    Ok(input)
}
