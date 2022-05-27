use bevy::prelude::{Query, Res, Text, Time, With};
use crate::models::ui::debug_console::DebugFpsCounter;

pub fn fps_counter_update_system(
    mut text_query: Query<&mut Text, With<DebugFpsCounter>>,
    time: Res<Time>,
) {
    if time.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[1].value = format!("{:#?}", 1.0 / time.delta_seconds());
        }
    }
}
