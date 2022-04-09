use bevy::app::EventReader;
use bevy::prelude::{Commands, Entity, ResMut, State, With};

use crate::{AppState, Player, Query, ToAppState};
use crate::models::events::player_died_event::PlayerDiedEvent;
use crate::resources::state_resources::AppStateTrigger;

pub fn player_died_system(
    mut commands: Commands,
    mut app_state : ResMut<State<AppState>>,
    mut state_trigger: ResMut<AppStateTrigger>,
    mut player_died_event: EventReader<PlayerDiedEvent>,
) {
    for event in player_died_event.iter() {
        commands.entity(event.player_entity).despawn();
        //app_state.set(AppState::GameOver).unwrap();
        state_trigger.State_Change_Trigger = ToAppState::ToGameOver;
        println!("player dieded")
    }
}