use bevy::prelude::*;

use crate::AppState;
use crate::scheduling::BaseSets;
use crate::world::background::background_startup_system::background_startup_system;
use crate::world::background::move_background_tiles_system::move_background_tiles_system;

mod background_startup_system;
mod move_background_tiles_system;

/// this plugin manages the "world" spawning and procedural world building
///
/// [ background_startup_system ] spawns the initial world tiles
///
/// [ move_background_tiles_system ] moves the background tiles when the player moves into
/// a new "chunk"
pub struct BackgroundPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct BackgroundUpdateSystemSet;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            BackgroundUpdateSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(in_state(AppState::InGame))
        );

        app.add_system(background_startup_system.in_schedule(OnEnter(AppState::MainMenu)));

        app.add_system(move_background_tiles_system.in_set(BackgroundUpdateSystemSet));
    }
}