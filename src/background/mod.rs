use bevy::prelude::Plugin;

use crate::{App, SetupStages};
use crate::background::background_startup_system::background_startup_system;
use crate::background::move_background_tiles_system::move_background_tiles_system;

pub mod background_startup_system;
pub mod move_background_tiles_system;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(SetupStages::PlayerSetup, background_startup_system)
            .add_system(move_background_tiles_system);
    }
}