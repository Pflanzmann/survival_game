use bevy::prelude::{App, Plugin};

use crate::input::player_control_system::player_control_system;
use crate::input::setup_camera_system::setup_camera_system;
use crate::SetupStages;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(SetupStages::AfterPlayerSetup, setup_camera_system)
            .add_system(player_control_system);
    }
}