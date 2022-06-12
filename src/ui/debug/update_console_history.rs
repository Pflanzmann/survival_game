use bevy::prelude::{DetectChanges, EventReader, Query, ResMut, Text, With};

use crate::models::events::debug_command_info_event::DebugCommandInfoEvent;
use crate::models::resources::console_history::ConsoleHistory;
use crate::models::ui::debug_console_history::DebugConsoleHistory;

pub fn update_console_history(
    mut debug_command_info_events: EventReader<DebugCommandInfoEvent>,
    mut console_history: ResMut<ConsoleHistory>,
    mut text_query: Query<&mut Text, With<DebugConsoleHistory>>,
) {
    for event in debug_command_info_events.iter() {
        console_history.log.insert(0, format!("    --> {}", event.debug_command.clone()));

        console_history.write_history_to_file();
    }

    if console_history.is_changed() {
        let mut history_string = String::new();
        for text in console_history.log.iter().rev() {
            history_string.push_str(text);
        }

        for mut text in text_query.iter_mut() {
            text.sections[0].value = history_string.to_string();
        }
    }
}