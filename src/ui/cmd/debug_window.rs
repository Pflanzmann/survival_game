use bevy::prelude::*;
use bevy_egui::*;
use bevy_egui::egui::*;

use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::models::resources::console_history::ConsoleHistory;

#[derive(Default)]
pub struct ConsoleState {
    pub input_text: String,
    pub y_scroll: f32,
}

pub fn show_debug_window_system(
    mut egui_context: EguiContexts,
    mut debug_event: EventWriter<DebugCommandEvent>,
    mut console_history: ResMut<ConsoleHistory>,
    mut state: Local<ConsoleState>,
    keys: Res<Input<KeyCode>>,
) {
    egui::Window::new("Console")
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::new(-10.0, 10.0))
        .fixed_size(egui::Vec2::new(400.0, 200.0))
        .show(egui_context.ctx_mut(), |ui| {
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for text in console_history.log.iter().rev() {
                        ui.label(text);
                    }
                });

            ui.horizontal_wrapped(|ui| {
                ui.label("Commands:");
                let input = ui.text_edit_singleline(&mut state.input_text);

                if input.lost_focus() {
                    ui.input(|_| {
                        if keys.pressed(KeyCode::LShift) {
                            input.request_focus();
                        }

                        console_history.send_command(&mut debug_event, state.input_text.clone());
                        state.input_text.clear();
                    });
                }

                if input.has_focus() && (keys.just_pressed(KeyCode::Up) || keys.just_pressed(KeyCode::Down)) {
                    state.input_text.clear();

                    if keys.just_pressed(KeyCode::Up) {
                        console_history.scroll_index += 1;
                    } else {
                        console_history.scroll_index -= 1;
                    }

                    console_history.scroll_index = console_history.scroll_index.clamp(0, console_history.command_history.len() as i16);

                    let current_history_text = if console_history.scroll_index >= 0 {
                        let index = usize::try_from(console_history.scroll_index).unwrap_or(0);

                        let index = usize::clamp(index - 1, 0, console_history.command_history.len());

                        match console_history.command_history.get(index) {
                            Some(text) => text,
                            None => "",
                        }
                    } else {
                        ""
                    };

                    state.input_text.push_str(current_history_text.trim());
                }
            });
        });
}
