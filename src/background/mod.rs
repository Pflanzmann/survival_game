use bevy::prelude::{App, Plugin, SystemSet};

use crate::AppState;
use crate::background::background_startup_system::background_startup_system;
use crate::background::move_background_tiles_system::move_background_tiles_system;
use crate::util::stage_label_helper::in_update;

mod background_startup_system;
mod move_background_tiles_system;

/// this plugin manages the "world" spawning and procedural world building
///
/// [ background_startup_system ] spawns the initial world tiles
///
/// [ move_background_tiles_system ] moves the background tiles when the player moves into
/// a new "chunk"

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
                    .with_system(background_startup_system)
            )
            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(move_background_tiles_system)
                )
            );
    }
}