use bevy::prelude::{App, Plugin, SystemSet};

use crate::AppState;
use crate::units::enemies::despawn_dead_enemy_system::despawn_dead_enemy_system;
use crate::units::enemies::despawn_far_enemy_system::despawn_far_enemy_system;
use crate::units::enemies::enemy_collision_dodge_system::enemy_collision_dodge_system;
use crate::util::stage_label_helper::{in_last, in_update};

pub mod despawn_dead_enemy_system;
pub mod despawn_far_enemy_system;
pub mod enemy_collision_dodge_system;


/// This plugin manages the everything related to [Enemy] systems and how they get applied.
///
/// The spawning of enemies and how the assignment of the move direction get handled in in
/// the [Update] of the [AppState::InGame]
///
/// The [despawn_dead_enemy_system] and [despawn_far_enemy_system] are called in the [Last]
/// because they despawn entities as a very last step so that other systems can
/// still react to the [PlayerDiedEvent]
pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(enemy_collision_dodge_system)
                )
            )

            .add_system_set(
                in_last(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(despawn_dead_enemy_system)
                        .with_system(despawn_far_enemy_system)
                )
            );
    }
}
