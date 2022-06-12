use bevy::prelude::*;

use crate::models::resources::ui_states::info_window_state::InfoWindowState;

pub fn show_info_fps_system(
    time: Res<Time>,
    mut state: ResMut<InfoWindowState>,
) {
    let text = format!("{:#?}", 1.0 / time.delta_seconds());
    state.infos.insert("fps".to_string(), text);
}