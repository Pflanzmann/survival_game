use bevy::prelude::*;

use crate::AppState;
use crate::scheduling::BaseSets;
use crate::world::background::move_background_tiles_system::move_background_tiles_system;
use crate::world::background::update_background_tiles_system::update_background_tiles_system;

mod move_background_tiles_system;
mod update_background_tiles_system;

/// this plugin manages the "world" spawning and procedural world building
///
/// [ update_background_tiles_system ] spawns new tiles if needed and reuses old ones if possible
/// when the background grid gets moved
///
/// [ move_background_tiles_system ] moves the center of the background grid when the player moves
/// into a new "chunk"
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

        app.add_system(move_background_tiles_system.in_set(BackgroundUpdateSystemSet));
        app.add_system(update_background_tiles_system.in_set(BackgroundUpdateSystemSet));
    }
}