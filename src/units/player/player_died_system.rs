use bevy::prelude::{Commands, EventReader, NextState, ResMut};

use crate::AppState;
use crate::models::events::player_died_event::PlayerDiedEvent;

pub fn player_died_system(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut player_died_event: EventReader<PlayerDiedEvent>,
) {
    for event in player_died_event.iter() {
        commands.entity(event.player_entity).despawn();
        next_state.set(AppState::GameOver)
    }
}