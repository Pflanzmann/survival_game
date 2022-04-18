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

        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", *string);
        }
    }

    if keys.just_pressed(KeyCode::Return) {
        if string.is_empty() {
            return;
        }

        debug_event.send(DebugCommandEvent { debug_command: string.clone() });
        string.clear();
        debug_history.scroll_index = 0;
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", *string);
        }

        return;
    }

    if keys.just_pressed(KeyCode::Up) {
        string.clear();

        debug_history.scroll_index -= 1;

        let current_history_text = match debug_history.history.get(debug_history.scroll_index % debug_history.history.len()) {
            Some(text) => text,
            None => return,
        };

        string.push_str(&*current_history_text.clone());
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", *string);
        }

        return;
    }

    if keys.just_pressed(KeyCode::Down) {
        string.clear();

        debug_history.scroll_index += 1;

        let current_history_text = match debug_history.history.get(debug_history.scroll_index % debug_history.history.len()) {
            Some(text) => text,
            None => return,
        };

        string.push_str(&*current_history_text.clone());
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", *string);
        }

        return;
    }

    if keys.just_pressed(KeyCode::Space) {
        string.push(' ');
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", *string);
        }
    }

    for ev in char_evr.iter() {
        if ev.char.is_alphabetic() {
            string.push(ev.char);
        }
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", *string);
        }
    }
}
