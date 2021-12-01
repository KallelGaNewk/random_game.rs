use std::{fs, str};

use regex::Regex;

pub struct Game {
    pub game_id: String,
    pub name: String,
}

// TODO: Implement name regex
pub fn parse_manifest(file_name: &str, steamapps_folder: &str) -> Game {
    let path = format!("{}\\{}", steamapps_folder, file_name);
    let buffer = fs::read(path).unwrap();
    let content = str::from_utf8(&buffer).unwrap();

    let appid_regex = Regex::new(r"\d+").unwrap();

    let appid = appid_regex
        .captures(content.split("\n").find(|i| i.contains("appid")).unwrap())
        .unwrap()
        .get(0)
        .unwrap()
        .as_str();

    Game {
        game_id: appid.to_string(),
        name: "Not functional yet...".to_string(),
    }
}

/*
    // Regex not working
    let name_regex = Regex::new("name.*\\\"(.+)\\\"'").unwrap();
    let name = name_regex.captures(content.split("\n").find(|i| i.contains("name")).unwrap()).unwrap().get(0).unwrap().as_str();
*/
