use bevy::prelude::Plugin;

use crate::{App, AppState, SetupStages, SystemSet};
use crate::background::background_startup_system::background_startup_system;
use crate::background::move_background_tiles_system::move_background_tiles_system;

pub mod background_startup_system;
pub mod move_background_tiles_system;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set_to_stage(SetupStages::PlayerSetup, SystemSet::on_enter(AppState::InGame).with_system(background_startup_system))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(move_background_tiles_system));
    }
}