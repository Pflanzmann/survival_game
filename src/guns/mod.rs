use bevy::prelude::Plugin;

use crate::{App, SetupStages};
use crate::guns::setup_gun_system::setup_gun_system;
use crate::guns::straight_basic_shot_system::straight_basic_shot_system;

pub mod straight_basic_shot_system;
pub mod setup_gun_system;

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(SetupStages::AfterPlayerSetup, setup_gun_system)
            .add_system(straight_basic_shot_system);
    }
}