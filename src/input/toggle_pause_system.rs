use bevy::prelude::{Input, KeyCode, NextState, Res, ResMut, Resource, State, Time};

use crate::{AppState, ConsoleState};

#[derive(Default, Resource)]
pub struct StateTimer(f32);

pub fn toggle_pause_system(
    input: Res<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    app_state: ResMut<State<AppState>>,
    mut next_console_state: ResMut<NextState<ConsoleState>>,
    console_state: Res<State<ConsoleState>>,
    mut state_timer: ResMut<StateTimer>,
    time: Res<Time>,
) {
    state_timer.0 += time.delta().as_secs_f32();
    if state_timer.0 < 0.2 {
        return;
    }

    if input.pressed(KeyCode::Escape) {
        state_timer.0 = 0.0;
        match app_state.0 {
            AppState::Pre => {}
            AppState::MainMenu => {}
            AppState::Loading => {}
            AppState::InGame => { next_app_state.set(AppState::Paused); }
            AppState::GameOver => {}
            AppState::Paused => { next_app_state.set(AppState::InGame); }
            AppState::Shop => {}
            AppState::GameWon => {}
        }
    }

    if input.pressed(KeyCode::Tab) {
        state_timer.0 = 0.0;

        match console_state.0 {
            ConsoleState::Shown => next_console_state.set(ConsoleState::Hidden),
            ConsoleState::Hidden => next_console_state.set(ConsoleState::Shown),
        }
    }
}