use bevy::prelude::{NextState, ResMut};

use crate::AppState;

pub fn trigger_enter_main_system(
    mut next_state: ResMut<NextState<AppState>>,
) {
    next_state.set(AppState::MainMenu);
}