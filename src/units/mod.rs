use bevy::prelude::{CoreStage, Plugin, SystemSet};
use bevy::prelude::CoreStage::PostUpdate;

use crate::{App, SetupStages};
use crate::units::despawn_dead_enemy_system::despawn_dead_enemy_system;
use crate::units::despawn_far_enemy_system::despawn_far_enemy_system;
use crate::units::enemy_movement_system::enemy_movement_system;
use crate::units::enemy_spawn_system::{enemy_spawn_system, SpawnTimer};
use crate::units::fit_sprite_to_size_system::fit_sprite_to_size_system;
use crate::units::healthbar_update_system::healthbar_update_system;
use crate::units::player_died_system::player_died_system;
use crate::units::player_hit_system::player_hit_system;
use crate::units::setup_player_healthbar_system::setup_health_bar;
use crate::units::setup_player_system::setup_player_system;
use crate::units::sprite_direction_system::sprite_direction_system;

pub mod enemy_spawn_system;
pub mod enemy_movement_system;
pub mod setup_player_system;
pub mod sprite_direction_system;
pub mod healthbar_update_system;
pub mod setup_player_healthbar_system;
pub mod fit_sprite_to_size_system;
pub mod despawn_dead_enemy_system;
pub mod player_hit_system;
pub mod player_died_system;
pub mod despawn_far_enemy_system;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(SetupStages::PlayerSetup, setup_player_system)
            .add_startup_system_to_stage(SetupStages::AfterPlayerSetup, setup_health_bar)
            .add_system_to_stage(CoreStage::Last, player_died_system)
            .add_system_to_stage(CoreStage::Last, despawn_dead_enemy_system)

            .add_system_to_stage(CoreStage::PreUpdate, player_hit_system)

            .add_system(enemy_spawn_system)
            .add_system(enemy_movement_system)
            .add_system(sprite_direction_system)
            .add_system(healthbar_update_system)
            .add_system(fit_sprite_to_size_system)
            .add_system(player_hit_system)
            .add_system(despawn_far_enemy_system)
            .init_resource::<SpawnTimer>();
    }
}