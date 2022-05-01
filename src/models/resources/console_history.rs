use std::fs;
use std::fs::File;
use std::io::ErrorKind;

use serde::Deserialize;
use serde::Serialize;

use crate::util::read_file_to_string::read_file_to_string;

const HISTORY_PATH: &str = "console_history.json";

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct ConsoleHistory {
    pub log: Vec<String>,
    pub command_history: Vec<String>,
    pub scroll_index: i16,
}

pub fn read_history_from_file() -> ConsoleHistory {
    let history_string = read_file_to_string(HISTORY_PATH);
    let history = serde_json::from_str::<ConsoleHistory>(&history_string);

    match history {
        Ok(history) => history,
        Err(_) => ConsoleHistory::default()
    }
}

impl ConsoleHistory {
    pub fn write_history_to_file(&self) {
        match File::open(HISTORY_PATH) {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(&HISTORY_PATH) {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                _ => return,
            },
        };

        let json: String = serde_json::to_string(&self).unwrap();
        fs::write(HISTORY_PATH, json).expect("Could store history to file.");
    }
}