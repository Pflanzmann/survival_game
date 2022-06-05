use bevy::prelude::{Plugin, SystemSet};

use crate::{App, AppState};
use crate::units::apply_damaged_component_system::apply_damage_component_system;
use crate::units::apply_hit_effect_system::{apply_hit_effect_sprite_atlas_system, apply_hit_effect_sprite_system};
use crate::units::behaviors::BehaviorPlugin;
use crate::units::clear_damaged_entities_system::clear_damaged_entities_system;
use crate::units::enemies::EnemiesPlugin;
use crate::units::guns::GunPlugin;
use crate::units::health_bar_update_system::healthbar_update_system;
use crate::units::hit_system::hit_system;
use crate::units::inherit_unit_size_system::inherit_unit_size_system;
use crate::units::knock_back_system::knock_back_system;
use crate::units::layerable_system::layerable_system;
use crate::units::mirror_aim_to_move_direction_system::mirror_aim_to_move_direction_system;
use crate::units::modifications::UnitModificationsPlugin;
use crate::units::move_unit_system::move_unit_system;
use crate::units::player::PlayerPlugin;
use crate::units::projectile::ProjectilePlugin;
use crate::units::rotate_unit_system::rotate_unit_system;
use crate::units::sprite_aim_rotate_system::sprite_aim_rotate_system;
use crate::units::sprite_flip_system::{sprite_atlas_flip_system, sprite_flip_system};
use crate::units::sprite_move_rotate_system::sprite_move_rotate_system;
use crate::units::time_alive_system::time_alive_system;
use crate::units::unit_push_system::unit_push_system;
use crate::units::unit_size_change_system::{unit_size_sprite_change_system, unit_size_texture_atlas_sprite_change_system};
use crate::util::stage_label_helper::{in_last, in_pre_update, in_update};

mod sprite_flip_system;
mod health_bar_update_system;
mod unit_size_change_system;
mod modifications;
mod move_unit_system;
mod player;
mod enemies;
mod behaviors;
mod sprite_move_rotate_system;
mod sprite_aim_rotate_system;
mod apply_damaged_component_system;
mod apply_hit_effect_system;
mod projectile;
mod rotate_unit_system;
mod clear_damaged_entities_system;
mod mirror_aim_to_move_direction_system;
mod hit_system;
mod layerable_system;
mod unit_push_system;
mod time_alive_system;
mod knock_back_system;
mod inherit_unit_size_system;

pub mod guns;


/// This plugin manages the everything related to [Unit] systems and how they get applied.
///
/// The [PlayerPlugin] is for systems specific to all player.
/// The [EnemiesPlugin] is for systems specific to all enemies.
///
/// every system related to units overall get called in the [Update] of the [AppState::InGame].
pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(UnitModificationsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ProjectilePlugin)
            .add_plugin(EnemiesPlugin)
            .add_plugin(BehaviorPlugin)
            .add_plugin(GunPlugin)

            .add_system_set(
                in_pre_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(hit_system)
                )
            )

            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(rotate_unit_system)
                        .with_system(sprite_flip_system)
                        .with_system(sprite_atlas_flip_system)
                        .with_system(sprite_move_rotate_system)
                        .with_system(sprite_aim_rotate_system)
                        .with_system(healthbar_update_system)
                        .with_system(unit_size_sprite_change_system)
                        .with_system(unit_size_texture_atlas_sprite_change_system)
                        .with_system(apply_damage_component_system)
                        .with_system(apply_hit_effect_sprite_system)
                        .with_system(apply_hit_effect_sprite_atlas_system)
                        .with_system(clear_damaged_entities_system)
                        .with_system(mirror_aim_to_move_direction_system)
                        .with_system(unit_push_system)
                        .with_system(time_alive_system)
                        .with_system(knock_back_system)
                        .with_system(inherit_unit_size_system)
                )
            )

            .add_system_set(
                in_last(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(move_unit_system)
                        .with_system(layerable_system)
                )
            );
    }
}
