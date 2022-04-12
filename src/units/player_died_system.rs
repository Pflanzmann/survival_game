use bevy::app::EventReader;
use bevy::prelude::{Commands, ResMut, State, With};

use crate::{Query, ToAppState};
use crate::models::events::player_died_event::PlayerDiedEvent;
use crate::models::resources::state_resources::AppStateTrigger;

pub fn player_died_system(
    mut commands: Commands,
    mut state_trigger: ResMut<AppStateTrigger>,
    mut player_died_event: EventReader<PlayerDiedEvent>,
) {
    for event in player_died_event.iter() {
        commands.entity(event.player_entity).despawn();
        state_trigger.State_Change_Trigger = ToAppState::ToGameOver;
    }
}