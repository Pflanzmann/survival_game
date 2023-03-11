use bevy::prelude::{IntoSystemConfig, IntoSystemSetConfig, Plugin, SystemSet};

use crate::App;
use crate::collision::calculate_hit_box_quad_tree_system::calculate_hit_box_quad_tree_system;
use crate::collision::calculate_item_quad_tree_system::calculate_item_quad_tree_system;
use crate::collision::calculate_solid_body_quad_tree_system::calculate_solid_body_quad_tree_system;
use crate::collision::enemy_hit_box_collision_system::enemy_hit_box_collision_system;
use crate::collision::enemy_player_collision_system::enemy_player_collision_system;
use crate::collision::enemy_solid_body_collision_system::enemy_solid_body_collision_system;
use crate::collision::item_player_collision_system::item_player_collision_system;
use crate::collision::solid_body_collision_system::solid_body_collision_system;
use crate::scheduling::BaseSets;

mod enemy_player_collision_system;
mod enemy_hit_box_collision_system;
mod solid_body_collision_system;
mod item_player_collision_system;
mod enemy_solid_body_collision_system;
mod calculate_solid_body_quad_tree_system;
mod calculate_item_quad_tree_system;
mod calculate_hit_box_quad_tree_system;

/// this has system running to check for collision between different game objects
///
/// in order to not loose any collision events, These are created in a separate "stage" before
/// the regular update ticks. This scheduling is also necessary to order the firing and reaction
/// to events as well as possible despawns of entities. Otherwise we ran into problems with events
/// that try to react on entities that don't exist anymore
pub struct CollisionPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CollisionSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CalculateTreesSystemSet;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            CollisionSystemSet
                .in_base_set(BaseSets::Collision)
        );

        app.configure_set(
            CalculateTreesSystemSet
                .in_base_set(BaseSets::Collision)
        );

        app
            .add_system(enemy_player_collision_system.in_set(CollisionSystemSet))
            .add_system(enemy_hit_box_collision_system.in_set(CollisionSystemSet))
            .add_system(enemy_solid_body_collision_system.in_set(CollisionSystemSet))
            .add_system(solid_body_collision_system.in_set(CollisionSystemSet))
            .add_system(item_player_collision_system.in_set(CollisionSystemSet));

        app
            .add_system(calculate_solid_body_quad_tree_system.in_set(CalculateTreesSystemSet))
            .add_system(calculate_hit_box_quad_tree_system.in_set(CalculateTreesSystemSet))
            .add_system(calculate_item_quad_tree_system.in_set(CalculateTreesSystemSet));
    }
}
