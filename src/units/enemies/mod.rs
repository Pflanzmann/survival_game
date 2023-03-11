use bevy::prelude::*;

use crate::AppState;
use crate::scheduling::BaseSets;
use crate::units::enemies::despawn_dead_enemy_system::despawn_dead_enemy_system;
use crate::units::enemies::despawn_far_enemy_system::despawn_far_enemy_system;

pub mod despawn_dead_enemy_system;
pub mod despawn_far_enemy_system;


/// This plugin manages the everything related to [Enemy] systems and how they get applied.
///
/// The spawning of enemies and how the assignment of the move direction get handled in in
/// the [Update] of the [AppState::InGame]
///
/// The [despawn_dead_enemy_system] and [despawn_far_enemy_system] are called in the [Last]
/// because they despawn entities as a very last step so that other systems can
/// still react to the [PlayerDiedEvent]
pub struct EnemiesPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct EnemiesSystemSet;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            EnemiesSystemSet
                .in_base_set(BaseSets::Last)
                .run_if(in_state(AppState::InGame))
        );

        app
            .add_system(despawn_dead_enemy_system.in_set(EnemiesSystemSet))
            .add_system(despawn_far_enemy_system.in_set(EnemiesSystemSet));
    }
}
