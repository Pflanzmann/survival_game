use bevy::app::CoreStage::PostUpdate;
use bevy::core::FixedTimestep;
use bevy::prelude::{Plugin, SystemSet};

use crate::{App, AppState};
use crate::collision::enemy_bullet_collision_system::enemy_bullet_collision_system;
use crate::collision::enemy_enemy_collision_system::enemy_enemy_collision_system;
use crate::collision::enemy_player_collision_system::enemy_player_collision_system;
use crate::collision::item_player_collision_system::item_player_collision_system;

pub mod enemy_player_collision_system;
pub mod enemy_bullet_collision_system;
pub mod enemy_enemy_collision_system;
pub mod item_player_collision_system;

pub struct CollisionPlugin;

const FIXED_TIMESTEP: f64 = 0.1;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            PostUpdate,
            SystemSet::on_update(AppState::InGame)
                .with_run_criteria(FixedTimestep::step(FIXED_TIMESTEP))
                .with_system(enemy_player_collision_system)
                .with_system(enemy_bullet_collision_system)
                .with_system(enemy_enemy_collision_system)
                .with_system(item_player_collision_system),
        );
    }
}
