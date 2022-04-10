use bevy::prelude::{ResMut, State};
use crate::{AppState, AppStateTrigger, ToAppState};

pub fn execute_state_switch_system(
    mut state_trigger: ResMut<AppStateTrigger>,
    mut app_state: ResMut<State<AppState>>,
) {
    match state_trigger.State_Change_Trigger {
        ToAppState::ToPre => {
            if app_state.current() != &AppState::Pre {
                state_trigger.State_Change_Trigger = ToAppState::None;
                app_state.set(AppState::Pre).unwrap();
            }
        }
        ToAppState::ToMainMenu => {
            if app_state.current() != &AppState::MainMenu {
                state_trigger.State_Change_Trigger = ToAppState::None;
                app_state.set(AppState::MainMenu).unwrap();
            }
        }
        ToAppState::ToLoading => {
            if app_state.current() != &AppState::Loading {
                state_trigger.State_Change_Trigger = ToAppState::None;
                app_state.set(AppState::Loading).unwrap();
            }
        }
        ToAppState::ToInGame => {
            if app_state.current() != &AppState::InGame {
                state_trigger.State_Change_Trigger = ToAppState::None;
                app_state.set(AppState::InGame).unwrap();
            }
        }
        ToAppState::ToGameOver => {
            if app_state.current() != &AppState::GameOver {
                state_trigger.State_Change_Trigger = ToAppState::None;
                app_state.set(AppState::GameOver).unwrap();
            }
        }
        ToAppState::ToPaused => {
            if app_state.current() != &AppState::Paused {
                state_trigger.State_Change_Trigger = ToAppState::None;
                app_state.set(AppState::Paused).unwrap();
            }
        }
        ToAppState::None => {}
    }
}