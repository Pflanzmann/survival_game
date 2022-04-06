use bevy::app::EventReader;
use bevy::prelude::{Commands, Entity, ResMut, State, With};

use crate::{AppState, Player, Query};
use crate::models::events::player_died_event::PlayerDiedEvent;

pub fn player_died_system(
    mut commands: Commands,
    mut app_state : ResMut<State<AppState>>,
    mut player_died_event: EventReader<PlayerDiedEvent>,
) {
    for event in player_died_event.iter() {
        commands.entity(event.player_entity).despawn();
        app_state.set(AppState::MainMenu).unwrap();
    }
}