use bevy::prelude::*;

use crate::{App, AppState};
use crate::scheduling::BaseSets;
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
use crate::units::level_up_system::level_up_system;
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
pub mod level_up_system;


/// This plugin manages the everything related to [Unit] systems and how they get applied.
///
/// The [PlayerPlugin] is for systems specific to all player.
/// The [EnemiesPlugin] is for systems specific to all enemies.
///
/// every system related to units overall get called in the [Update] of the [AppState::InGame].
pub struct UnitPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UnitHitSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UnitUpdateSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UnitMovementSystemSet;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            UnitHitSystemSet
                .in_base_set(BaseSets::PreUpdate)
                .run_if(in_state(AppState::InGame))
        );

        app.configure_set(
            UnitUpdateSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(in_state(AppState::InGame))
        );

        app.configure_set(
            UnitMovementSystemSet
                .in_base_set(BaseSets::Last)
                .run_if(in_state(AppState::InGame))
        );

        app
            .add_plugin(UnitModificationsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ProjectilePlugin)
            .add_plugin(EnemiesPlugin)
            .add_plugin(BehaviorPlugin)
            .add_plugin(GunPlugin);

        app.add_system(hit_system.in_set(UnitHitSystemSet));

        app
            .add_system(rotate_unit_system.in_set(UnitUpdateSystemSet))
            .add_system(sprite_flip_system.in_set(UnitUpdateSystemSet))
            .add_system(sprite_atlas_flip_system.in_set(UnitUpdateSystemSet))
            .add_system(sprite_move_rotate_system.in_set(UnitUpdateSystemSet))
            .add_system(sprite_aim_rotate_system.in_set(UnitUpdateSystemSet))
            .add_system(healthbar_update_system.in_set(UnitUpdateSystemSet))
            .add_system(unit_size_sprite_change_system.in_set(UnitUpdateSystemSet))
            .add_system(unit_size_texture_atlas_sprite_change_system.in_set(UnitUpdateSystemSet))
            .add_system(apply_damage_component_system.in_set(UnitUpdateSystemSet))
            .add_system(apply_hit_effect_sprite_system.in_set(UnitUpdateSystemSet))
            .add_system(apply_hit_effect_sprite_atlas_system.in_set(UnitUpdateSystemSet))
            .add_system(clear_damaged_entities_system.in_set(UnitUpdateSystemSet))
            .add_system(mirror_aim_to_move_direction_system.in_set(UnitUpdateSystemSet))
            .add_system(unit_push_system.in_set(UnitUpdateSystemSet))
            .add_system(time_alive_system.in_set(UnitUpdateSystemSet))
            .add_system(knock_back_system.in_set(UnitUpdateSystemSet))
            .add_system(inherit_unit_size_system.in_set(UnitUpdateSystemSet))
            .add_system(level_up_system.in_set(UnitUpdateSystemSet));

        app
            .add_system(move_unit_system.in_set(UnitMovementSystemSet))
            .add_system(layerable_system.in_set(UnitMovementSystemSet));
    }
}
