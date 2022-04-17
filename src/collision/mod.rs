use bevy::core::FixedTimestep;
use bevy::prelude::{Plugin, SystemSet};

use crate::{App, AppState};
use crate::collision::enemy_bullet_collision_system::enemy_bullet_collision_system;
use crate::collision::enemy_enemy_collision_system::enemy_enemy_collision_system;
use crate::collision::enemy_player_collision_system::enemy_player_collision_system;
use crate::collision::item_player_collision_system::item_player_collision_system;
use crate::util::stage_label_helper::in_collision;

mod enemy_player_collision_system;
mod enemy_bullet_collision_system;
mod enemy_enemy_collision_system;
mod item_player_collision_system;

/// this has system running to check for collision between different game objects
///
/// in order to not loose any collision events, These are created in a separate "stage" before
/// the regular update ticks. This scheduling is also necessary to order the firing and reaction
/// to events as well as possible despawns of entities. Otherwise we ran into problems with events
/// that try to react on entities that don't exist anymore

pub struct CollisionPlugin;

const FIXED_TIMESTEP: f64 = 0.02;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            in_collision(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(FIXED_TIMESTEP))
                    .with_system(enemy_player_collision_system)
                    .with_system(enemy_bullet_collision_system)
                    .with_system(enemy_enemy_collision_system)
                    .with_system(item_player_collision_system),
            )
        );
    }
}
