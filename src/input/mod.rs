use bevy::prelude::{App, CoreStage, Plugin};

use crate::{camera_system, SetupStages};
use crate::input::camera_systems::setup_camera_system;
use crate::input::gun_mod_debug_system::gun_mod_debug_system;
use crate::input::player_control_aim_system::player_control_aim_system;
use crate::input::player_control_movement_system::player_control_movement_system;

pub mod player_control_movement_system;
pub mod camera_systems;
pub mod gun_mod_debug_system;
pub mod player_control_aim_system;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(SetupStages::AfterPlayerSetup, setup_camera_system)
            .add_system(player_control_movement_system)
            .add_system(player_control_aim_system)
            .add_system(gun_mod_debug_system);
    }
}