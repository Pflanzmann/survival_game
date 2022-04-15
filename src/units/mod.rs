use bevy::prelude::{Plugin, SystemSet};

use enemies::despawn_dead_enemy_system::despawn_dead_enemy_system;
use enemies::despawn_far_enemy_system::despawn_far_enemy_system;
use enemies::enemy_movement_system::enemy_set_move_direction_system;
use enemies::enemy_spawn_system::{enemy_spawn_system, SpawnTimer};
use player::player_died_system::player_died_system;
use player::player_hit_system::player_hit_system;
use player::setup_player_health_bar_system::setup_player_health_bar_system;
use player::setup_player_system::setup_player_system;

use crate::{App, AppState};
use crate::units::behaviours::BehaviourPlugin;
use crate::units::enemies::EnemiesPlugin;
use crate::units::fit_sprite_to_size_system::fit_sprite_to_size_system;
use crate::units::healthbar_update_system::healthbar_update_system;
use crate::units::move_unit_system::move_unit_system;
use crate::units::player::PlayerPlugin;
use crate::units::sprite_direction_system::sprite_direction_system;
use crate::units::unit_modifications::UnitModificationsPlugin;
use crate::util::stage_label_helper::{in_last, in_pre_update, in_update};

mod sprite_direction_system;
mod healthbar_update_system;
mod fit_sprite_to_size_system;
mod unit_modifications;
mod move_unit_system;
mod player;
mod enemies;
mod behaviours;

pub struct UnitPlugin;

/// This plugin manages the everything related to [Unit] systems and how they get applied.
///
/// The [PlayerPlugin] is for systems specific to all player.
/// The [EnemiesPlugin] is for systems specific to all enemies.
///
/// every system related to units overall get called in the [Update] of the [AppState::InGame].
impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(UnitModificationsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemiesPlugin)
            .add_plugin(BehaviourPlugin)

            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(move_unit_system)
                        .with_system(sprite_direction_system)
                        .with_system(healthbar_update_system)
                        .with_system(fit_sprite_to_size_system)
                )
            );
    }
}
