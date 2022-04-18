use bevy::prelude::{EventReader, Query, ResMut, Text, With};

use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::models::resources::console_history::ConsoleHistory;
use crate::models::ui_components::debug_console::DebugConsoleHistory;

pub fn update_console_history(
    mut debug_command_events: EventReader<DebugCommandEvent>,
    mut console_history: ResMut<ConsoleHistory>,
    mut text_query: Query<&mut Text, With<DebugConsoleHistory>>,
) {
    for event in debug_command_events.iter() {
        console_history.history.push(event.debug_command.clone());

        let mut history_string = String::new();
        for text in console_history.history.iter() {
            history_string.push_str(text);
            history_string.push_str("\n");
        }

        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", history_string);
        }
    }
}