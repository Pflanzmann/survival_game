use bevy::prelude::{Plugin, SystemSet};

use crate::{App, AppState};
use crate::collision::calculate_quad_tree_system::calculate_quad_tree_system;
use crate::collision::enemy_hit_box_collision_system::enemy_hit_box_collision_system;
use crate::collision::enemy_player_collision_system::enemy_player_collision_system;
use crate::collision::enemy_solid_body_collision_system::enemy_solid_body_collision_system;
use crate::collision::item_player_collision_system::item_player_collision_system;
use crate::collision::solid_body_collision_system::solid_body_collision_system;
use crate::util::stage_label_helper::{in_collision, in_update};

mod enemy_player_collision_system;
mod enemy_hit_box_collision_system;
mod solid_body_collision_system;
mod item_player_collision_system;
mod calculate_quad_tree_system;
mod enemy_solid_body_collision_system;

/// this has system running to check for collision between different game objects
///
/// in order to not loose any collision events, These are created in a separate "stage" before
/// the regular update ticks. This scheduling is also necessary to order the firing and reaction
/// to events as well as possible despawns of entities. Otherwise we ran into problems with events
/// that try to react on entities that don't exist anymore
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_collision(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(enemy_player_collision_system)
                        .with_system(enemy_hit_box_collision_system)
                        .with_system(enemy_solid_body_collision_system)
                        .with_system(solid_body_collision_system)
                        .with_system(item_player_collision_system)
                )
            )

            .add_system_set(
                in_update(
                    SystemSet::new()
                        .with_system(calculate_quad_tree_system)
                )
            )
        ;
    }
}
