use bevy::prelude::{CoreStage, Plugin, SystemSet};

use crate::{App, AppState};
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
            .init_resource::<SpawnTimer>()

            .add_system_set(SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup_player_system)
            )

            .add_system_set(SystemSet::on_exit(AppState::MainMenu)
                .with_system(setup_health_bar)
            )

            .add_system_set_to_stage(CoreStage::Last, SystemSet::on_update(AppState::InGame)
                .with_system(player_died_system)
                .with_system(despawn_dead_enemy_system),
            )

            .add_system_set(SystemSet::on_update(AppState::InGame)
                .with_system(enemy_spawn_system)
                .with_system(enemy_movement_system)
                .with_system(sprite_direction_system)
                .with_system(healthbar_update_system)
                .with_system(fit_sprite_to_size_system)
                .with_system(despawn_far_enemy_system)
            )
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .with_system(player_hit_system),
            );
    }
}