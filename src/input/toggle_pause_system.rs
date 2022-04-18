use bevy::prelude::{Input, KeyCode, Res, ResMut, State, Time};

use ConsoleState::Shown;

use crate::{AppState, AppStateTrigger, ConsoleState, ToAppState};
use crate::ConsoleState::Hidden;

#[derive(Default)]
pub struct StateTimer(f32);

pub fn toggle_pause_system(
    input: Res<Input<KeyCode>>,
    mut state_trigger: ResMut<AppStateTrigger>,
    app_state: ResMut<State<AppState>>,
    mut console_state: ResMut<State<ConsoleState>>,
    mut state_timer: ResMut<StateTimer>,
    time: Res<Time>,
) {
    state_timer.0 += time.delta().as_secs_f32();
    if state_timer.0 < 0.2 {
        return;
    }

    if input.pressed(KeyCode::Escape) {
        state_timer.0 = 0.0;
        match app_state.current() {
            AppState::Pre => {}
            AppState::MainMenu => {}
            AppState::Loading => {}
            AppState::InGame => { state_trigger.state_change_trigger = ToAppState::ToPaused; }
            AppState::GameOver => {}
            AppState::Paused => { state_trigger.state_change_trigger = ToAppState::ToInGame; }
            AppState::Shop => {}
        }
    }

    if input.pressed(KeyCode::Tab) {
        state_timer.0 = 0.0;
        match console_state.current() {
            Shown => console_state.set(Hidden),
            Hidden => console_state.set(Shown),
        };
    }
}