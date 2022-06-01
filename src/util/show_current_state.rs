use bevy::prelude::{Res, State};

use crate::AppState;

pub fn show_current_state(
    app_state: Res<State<AppState>>
) {
    match app_state.current() {
        AppState::Pre => { println!("Pre") }
        AppState::MainMenu => { println!("MainMenu") }
        AppState::Loading => { println!("Loading") }
        AppState::InGame => { println!("Ingame") }
        AppState::GameOver => { println!("GameOver") }
        AppState::Paused => { println!("Paused") }
        AppState::Shop => { println!("Shop") }
        AppState::GameWon => { println!("GameWon") }
    }
}