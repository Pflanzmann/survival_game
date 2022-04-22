use bevy::core::FixedTimestep;
use bevy::prelude::{Plugin, SystemSet};

use crate::{App, AppState};
use crate::collision::calculate_quad_tree_system::calculate_quad_tree_system;
use crate::collision::enemy_bullet_collision_system::enemy_bullet_collision_system;
use crate::collision::enemy_enemy_collision_system::enemy_enemy_collision_system;
use crate::collision::enemy_enemy_collision_system_old::enemy_enemy_collision_system_old;
use crate::collision::enemy_player_collision_system::enemy_player_collision_system;
use crate::collision::item_player_collision_system::item_player_collision_system;
use crate::collision::quad_tree::Quadtree;
use crate::collision::show_quad_tree_system::show_quad_tree_system;
use crate::util::stage_label_helper::{in_collision, in_update};

mod enemy_player_collision_system;
mod enemy_bullet_collision_system;
mod enemy_enemy_collision_system;
mod item_player_collision_system;
mod calculate_quad_tree_system;
mod quad_tree;
mod enemy_enemy_collision_system_old;
mod show_quad_tree_system;

/// this has system running to check for collision between different game objects
///
/// in order to not loose any collision events, These are created in a separate "stage" before
/// the regular update ticks. This scheduling is also necessary to order the firing and reaction
/// to events as well as possible despawns of entities. Otherwise we ran into problems with events
/// that try to react on entities that don't exist anymore

pub struct CollisionPlugin;

const FIXED_TIMESTEP: f64 = 0.02;

#[derive(Default)]
pub struct QuadTreeHolder {
    quad_tree: Quadtree,
}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<QuadTreeHolder>()

            .add_system_set(
                in_collision(
                    SystemSet::on_update(AppState::InGame)
                        .with_run_criteria(FixedTimestep::step(0.2))
                        .with_system(enemy_player_collision_system)
                        .with_system(enemy_bullet_collision_system)
                        .with_system(enemy_enemy_collision_system)
                        .with_system(enemy_enemy_collision_system_old)
                        .with_system(item_player_collision_system)
                        .with_system(show_quad_tree_system)
                )
            )

            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_run_criteria(FixedTimestep::step(0.2))
                        .with_system(calculate_quad_tree_system)
                )
            )
        ;
    }
}
