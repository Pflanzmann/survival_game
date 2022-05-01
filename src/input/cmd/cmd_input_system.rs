use bevy::prelude::{EventReader, EventWriter, Input, KeyCode, Local, Query, ReceivedCharacter, Res, ResMut, Text, With};

use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::models::resources::console_history::ConsoleHistory;
use crate::models::ui_components::debug_console::DebugConsoleInput;

pub fn cmd_input_system(
    mut char_evr: EventReader<ReceivedCharacter>,
    mut debug_event: EventWriter<DebugCommandEvent>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
    mut debug_history: ResMut<ConsoleHistory>,
    mut text_query: Query<&mut Text, With<DebugConsoleInput>>,
) {
    if keys.just_pressed(KeyCode::Back) {
        string.pop();
        debug_history.scroll_index = 0;

        for mut text in text_query.iter_mut() {
            text.sections[0].value = (*string).to_string();
        }

        return;
    }

    if keys.just_pressed(KeyCode::Return) {
        if string.is_empty() {
            return;
        }

        debug_event.send(DebugCommandEvent { debug_command: string.clone() });
        string.clear();
        debug_history.scroll_index = 0;
        for mut text in text_query.iter_mut() {
            text.sections[0].value = (*string).to_string();
        }

        return;
    }

    if keys.just_pressed(KeyCode::Up) || keys.just_pressed(KeyCode::Down) {
        string.clear();

        if keys.just_pressed(KeyCode::Up) {
            debug_history.scroll_index += 1;
        } else {
            debug_history.scroll_index -= 1;
        }

        debug_history.scroll_index = debug_history.scroll_index.clamp(0, debug_history.command_history.len() as i16);

        let current_history_text = if debug_history.scroll_index >= 0 {
            let index = usize::try_from(debug_history.scroll_index).unwrap_or(0);

            let index = usize::clamp(index - 1, 0, debug_history.command_history.len());

            match debug_history.command_history.get(index) {
                Some(text) => text,
                None => "",
            }
        } else {
            ""
        };

        string.push_str(current_history_text.trim());
        for mut text in text_query.iter_mut() {
            text.sections[0].value = (*string).to_string();
        }

        return;
    }

    for ev in char_evr.iter() {
        if ev.char.is_ascii() {
            string.push(ev.char);
        }

        debug_history.scroll_index = 0;
        for mut text in text_query.iter_mut() {
            text.sections[0].value = (*string).to_string();
        }
    }
}
