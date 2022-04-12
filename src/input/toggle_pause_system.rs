use bevy::prelude::{Input, KeyCode, Res, ResMut, State, Time};

use crate::{AppState, AppStateTrigger, ToAppState};

#[derive(Default)]
pub struct StateTimer(f32);

pub fn toggle_pause_system(
    input: Res<Input<KeyCode>>,
    mut state_trigger: ResMut<AppStateTrigger>,
    app_state: ResMut<State<AppState>>,
    mut state_timer: ResMut<StateTimer>,
    time: Res<Time>,
) {
    state_timer.0 += time.delta().as_secs_f32();
    if state_timer.0 < 0.2 {
        return;
    }

    if input.pressed(KeyCode::Space) {
        state_timer.0 = 0.0;
        match app_state.current() {
            AppState::Pre => {}
            AppState::MainMenu => {}
            AppState::Loading => {}
            AppState::InGame => { state_trigger.State_Change_Trigger = ToAppState::ToPaused; }
            AppState::GameOver => {}
            AppState::Paused => { state_trigger.State_Change_Trigger = ToAppState::ToInGame; }
            AppState::Shop => {}
        }
    }
}