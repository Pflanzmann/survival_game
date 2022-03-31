use bevy::prelude::Plugin;

use crate::{App, SetupStages};
use crate::units::enemy_movement_system::enemy_movement_system;
use crate::units::enemy_spawn_system::{enemy_spawn_system, SpawnTimer};
use crate::units::setup_player_system::setup_player_system;
use crate::units::sprite_direction_system::sprite_direction_system;
use crate::units::healthbar_update_system::healthbar_update_system;
use crate::units::setup_player_healthbar_system::setup_health_bar;


pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(SetupStages::PlayerSetup, setup_player_system)
            .add_startup_system_to_stage(SetupStages::AfterPlayerSetup, setup_health_bar)
            .add_system(enemy_spawn_system)
            .add_system(enemy_movement_system)
            .add_system(sprite_direction_system)
            .add_system(healthbar_update_system)
            .init_resource::<SpawnTimer>();
    }
}