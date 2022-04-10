use bevy::prelude::{ResMut, State};
use crate::AppState;

pub fn show_current_state(
    mut appstate: ResMut<State<AppState>>
){
    match appstate.current() {
        AppState::Pre => {println!("Pre")}
        AppState::MainMenu => {println!("MainMenu")}
        AppState::Loading => {println!("Loading")}
        AppState::InGame => {println!("Ingame")}
        AppState::GameOver => {println!("GameOver")}
        AppState::Paused => {println!("Paused")}
    }
}