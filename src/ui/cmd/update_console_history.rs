use bevy::prelude::{EventReader, Query, ResMut, Text, With};

use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::models::events::debug_command_info_event::DebugCommandInfoEvent;
use crate::models::resources::console_history::ConsoleHistory;
use crate::models::ui_components::debug_console::DebugConsoleHistory;

pub fn update_console_history(
    mut debug_command_events: EventReader<DebugCommandEvent>,
    mut debug_command_info_events: EventReader<DebugCommandInfoEvent>,
    mut console_history: ResMut<ConsoleHistory>,
    mut text_query: Query<&mut Text, With<DebugConsoleHistory>>,
) {
    for event in debug_command_events.iter() {
        console_history.command_history.insert(0, event.debug_command.clone());
        console_history.log.insert(0, format!("{}\n", event.debug_command.clone()));

        if console_history.log.len() > 30 {
            console_history.log.pop();
            console_history.command_history.pop();
        }

        let mut history_string = String::new();
        for text in console_history.log.iter().rev() {
            history_string.push_str(text);
        }

        for mut text in text_query.iter_mut() {
            text.sections[0].value = history_string.to_string();
        }

        console_history.write_history_to_file();
    }

    for event in debug_command_info_events.iter() {
        console_history.log.insert(0, format!("    --> {}\n", event.debug_command.clone()));

        let mut history_string = String::new();
        for text in console_history.log.iter().rev() {
            history_string.push_str(text);
        }

        for mut text in text_query.iter_mut() {
            text.sections[0].value = history_string.to_string();
        }

        if console_history.log.len() > 30 {
            console_history.log.pop();
        }

        console_history.write_history_to_file();
    }
}