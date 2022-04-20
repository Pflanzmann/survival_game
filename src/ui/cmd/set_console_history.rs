use bevy::prelude::{EventReader, Query, ResMut, Text, With};

use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::models::events::debug_command_info_event::DebugCommandInfoEvent;
use crate::models::resources::console_history::ConsoleHistory;
use crate::models::ui_components::debug_console::DebugConsoleHistory;

pub fn set_console_history(
    mut console_history: ResMut<ConsoleHistory>,
    mut text_query: Query<&mut Text, With<DebugConsoleHistory>>,
) {
    let mut history_string = String::new();
    for text in console_history.history.iter() {
        history_string.push_str(text);
    }

    for mut text in text_query.iter_mut() {
        text.sections[0].value = history_string.to_string();
    }
}