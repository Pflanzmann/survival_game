use bevy::prelude::{in_state, IntoSystemConfig, IntoSystemSetConfig, on_event, Plugin, SystemSet};

use acid_puddle_system::acid_puddle_system;
use helper::assign_attribute_to_projectile_system::assign_attribute_to_projectile_system;
use helper::assign_modification_to_projectile_system::assign_modification_to_projectile_system;
use helper::assign_with_associate_component_to_projectile_system::assign_with_associate_component_to_projectile_system;

use crate::{App, AppState};
use crate::models::events::projectile_shot_event::ProjectileShotEvent;
use crate::models::knock_back::KnockBack;
use crate::models::modifications::acid_puddle::{AcidPuddle, AcidPuddleOwner};
use crate::models::modifications::burning_shot::BurningShot;
use crate::models::modifications::curve_shot::CurveShot;
use crate::models::modifications::explosion_shot::ExplosionShot;
use crate::models::modifications::gravity_shot::GravityShot;
use crate::models::modifications::grow_shot::GrowShot;
use crate::models::modifications::knock_back_shot::KnockBackShot;
use crate::models::modifications::lightning::Lightning;
use crate::models::modifications::split_shot::SplitShot;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::hit_limit::HitLimit;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::models::unit_attributes::unit_size::UnitSize;
use crate::scheduling::BaseSets;
use crate::units::modifications::projectile_modifications::burning_shot_system::burning_shot_system;
use crate::units::modifications::projectile_modifications::curve_shot_system::curve_shot_system;
use crate::units::modifications::projectile_modifications::explosion_shot_system::explosion_shot_system;
use crate::units::modifications::projectile_modifications::gravity_shot::gravity_shot_system;
use crate::units::modifications::projectile_modifications::grow_shot_system::grow_shot_system;
use crate::units::modifications::projectile_modifications::helper::enable_projectile_collision::enable_projectile_collision;
use crate::units::modifications::projectile_modifications::lightning_system::lightning_system;
use crate::units::modifications::projectile_modifications::split_shot_system::split_shot_system;

mod curve_shot_system;
mod grow_shot_system;
mod split_shot_system;
mod gravity_shot;
mod explosion_shot_system;
mod helper;
mod lightning_system;
mod acid_puddle_system;
mod burning_shot_system;

/// This plugin manages the [Projectile]-[Modification]s and how they get applied.
/// All the systems get added in the [PreUpdate] so that they can react last to all
/// other accesses of the projectile.
///
/// All system get only used in the [AppState::Ingame].
pub struct ProjectileModificationsPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct AssignProjectileModsToBulletSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct RunProjectileModsToBulletSystemSet;

impl Plugin for ProjectileModificationsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            AssignProjectileModsToBulletSystemSet
                .in_base_set(BaseSets::PreUpdate)
                .run_if(on_event::<ProjectileShotEvent>())
        );

        app.configure_set(
            RunProjectileModsToBulletSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(on_event::<ProjectileShotEvent>())
                .run_if(in_state(AppState::InGame))
        );

        app
            .add_system(enable_projectile_collision.in_set(AssignProjectileModsToBulletSystemSet))
            .add_system(assign_attribute_to_projectile_system::<Damage>.in_set(AssignProjectileModsToBulletSystemSet))
            .add_system(assign_attribute_to_projectile_system::<HitLimit>.in_set(AssignProjectileModsToBulletSystemSet))
            .add_system(assign_attribute_to_projectile_system::<MoveSpeed>.in_set(AssignProjectileModsToBulletSystemSet))
            .add_system(assign_attribute_to_projectile_system::<TravelRange>.in_set(AssignProjectileModsToBulletSystemSet))
            .add_system(assign_attribute_to_projectile_system::<UnitSize>.in_set(AssignProjectileModsToBulletSystemSet))

            .add_system(assign_modification_to_projectile_system::<CurveShot>.in_set(AssignProjectileModsToBulletSystemSet))
            .add_system(assign_modification_to_projectile_system::<GrowShot>.in_set(AssignProjectileModsToBulletSystemSet))
            .add_system(assign_modification_to_projectile_system::<SplitShot>.in_set(AssignProjectileModsToBulletSystemSet))
            .add_system(assign_modification_to_projectile_system::<GravityShot>.in_set(AssignProjectileModsToBulletSystemSet))
            .add_system(assign_with_associate_component_to_projectile_system::<KnockBackShot, KnockBack>.in_set(AssignProjectileModsToBulletSystemSet))
            .add_system(assign_modification_to_projectile_system::<ExplosionShot>.in_set(AssignProjectileModsToBulletSystemSet))
            .add_system(assign_modification_to_projectile_system::<Lightning>.in_set(AssignProjectileModsToBulletSystemSet))
            .add_system(assign_modification_to_projectile_system::<AcidPuddle>.in_set(AssignProjectileModsToBulletSystemSet))
            .add_system(assign_modification_to_projectile_system::<AcidPuddleOwner>.in_set(AssignProjectileModsToBulletSystemSet))
            .add_system(assign_modification_to_projectile_system::<BurningShot>.in_set(AssignProjectileModsToBulletSystemSet));

        app
            .add_system(curve_shot_system.in_set(RunProjectileModsToBulletSystemSet))
            .add_system(grow_shot_system.in_set(RunProjectileModsToBulletSystemSet))
            .add_system(split_shot_system.in_set(RunProjectileModsToBulletSystemSet))
            .add_system(gravity_shot_system.in_set(RunProjectileModsToBulletSystemSet))
            .add_system(explosion_shot_system.in_set(RunProjectileModsToBulletSystemSet))
            .add_system(lightning_system.in_set(RunProjectileModsToBulletSystemSet))
            .add_system(acid_puddle_system.in_set(RunProjectileModsToBulletSystemSet))
            .add_system(burning_shot_system.in_set(RunProjectileModsToBulletSystemSet));
    }
}
