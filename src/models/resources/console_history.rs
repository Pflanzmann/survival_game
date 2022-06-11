use std::fs;
use std::fs::File;
use std::io::ErrorKind;

use bevy::prelude::{EventWriter, Resource};
use serde::Deserialize;
use serde::Serialize;

use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::util::read_file_to_string::read_file_to_string;

const HISTORY_PATH: &str = "console_history.json";

#[derive(Default, Deserialize, Serialize, Clone, Resource)]
pub struct ConsoleHistory {
    pub log: Vec<String>,
    pub command_history: Vec<String>,
    pub scroll_index: i16,
}

pub fn read_history_from_file() -> ConsoleHistory {
    let history_string = read_file_to_string(HISTORY_PATH);
    let history = serde_json::from_str::<ConsoleHistory>(&history_string);

    match history {
        Ok(mut history) => {
            history.scroll_index = 0;
            history
        }
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

    pub fn send_command(&mut self, event_writer: &mut EventWriter<DebugCommandEvent>, command: String) {
        if command.is_empty() {
            return;
        }

        let debug_command = command.trim().to_lowercase().to_string();
        let commands = debug_command.split('&');

        for command in commands {
            let debug_command = command.trim().to_string();
            let mut cmd_split = debug_command.split_whitespace();

            let key = match cmd_split.next() {
                Some(value) => value.to_lowercase().trim().to_string(),
                None => String::new(),
            };

            let mut arguments: Vec<String> = Vec::new();
            let mut values: Vec<String> = Vec::new();

            for input_string in cmd_split {
                if input_string.contains('-') {
                    arguments.push(input_string.to_lowercase().trim().to_string())
                } else {
                    values.push(input_string.to_lowercase().trim().to_string())
                }
            }

            event_writer.send(DebugCommandEvent {
                debug_command,

                key,
                values,
                arguments,
            });
        }

        let index = self.command_history.iter().position(|x| *x == debug_command);
        if let Some(index) = index {
            self.command_history.remove(index);
        }

        self.command_history.insert(0, debug_command.clone());
        self.log.insert(0, debug_command.clone());

        while self.command_history.len() > 30 {
            self.command_history.pop();
        }

        while self.log.len() > 30 {
            self.log.pop();
        }

        self.scroll_index = 0;
        self.write_history_to_file();
    }
}