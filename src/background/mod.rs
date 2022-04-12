use bevy::prelude::Plugin;

use crate::{App, AppState, SetupStages, SystemSet};
use crate::background::background_startup_system::background_startup_system;
use crate::background::move_background_tiles_system::move_background_tiles_system;
use crate::util::stage_label_helper::in_update;

mod background_startup_system;
mod move_background_tiles_system;

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