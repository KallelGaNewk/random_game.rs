use crate::manifest_parser::parse_manifest;
use clap::{App, Arg};
use manifest_parser::Game;
use rand::prelude::SliceRandom;
use std::{fs, process::exit};
use which::which;

mod manifest_parser;

const STEAM_BIN_PATH: &str = r#"C:\Program Files (x86)\Steam\steam.exe"#;
const STEAMAPPS_FOLDER: &str = r#"C:\Program Files (x86)\Steam\steamapps"#;

fn main() {
    let config = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("KallelGaNewk")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("launch")
                .short("l")
                .help("Launches the game when picked."),
        )
        .get_matches();

    let (game_id, name) = pick_game();

    println!("Game: {}", name);

    if config.is_present("launch") {
        println!("Lauching...");
        launch_app(game_id);
    }
}

fn pick_game() -> (String, String) {
    let has_steam = which(STEAM_BIN_PATH).is_ok();
    if !has_steam {
        println!("Not found steam in {} path.", STEAM_BIN_PATH);
        exit(1);
    }

    let mut rng = rand::thread_rng();
    let mut games = get_games();
    games.shuffle(&mut rng);

    let choosed_game = games.first().unwrap();

    (
        choosed_game.game_id.to_owned(),
        choosed_game.name.to_owned(),
    )
}

fn get_games() -> Vec<Game> {
    let steamapps_raw = fs::read_dir(STEAMAPPS_FOLDER).unwrap();

    steamapps_raw
        .filter(|entry| {
            entry
                .as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .ends_with(".acf")
        })
        .map(|i| {
            let file_name = i.unwrap().file_name().to_str().unwrap().to_owned();
            parse_manifest(&file_name, STEAMAPPS_FOLDER)
        })
        .collect::<Vec<Game>>()
}

fn launch_app(appid: String) {
    let url = format!("steam://rungameid/{}", appid);

    std::process::Command::new(STEAM_BIN_PATH)
        .arg(url)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .ok();
}
