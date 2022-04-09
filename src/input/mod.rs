use bevy::prelude::{App, CoreStage, Plugin};

use crate::{AppState, camera_system, SetupStages, SystemSet};
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
            .add_system_set(SystemSet::on_enter(AppState::InGame)
                .with_system(setup_camera_system)
            )

            .add_system_set(SystemSet::on_update(AppState::InGame)
                                .with_system(player_control_movement_system)
                                .with_system(player_control_aim_system)
                                .with_system(gun_mod_debug_system)
            );
    }
}