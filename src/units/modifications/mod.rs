use bevy::app::Plugin;
use bevy::prelude::*;

use helper::apply_affect_system::apply_affect_system;
use helper::apply_player_mod_to_target_system::apply_player_mod_to_target_system;
use helper::apply_projectile_mod_to_targets_gun_system::apply_projectile_mod_to_targets_gun_system;
use helper::remove_affect_system::remove_affect_system;
use helper::remove_player_mod_from_target_system::remove_player_mod_from_target_system;
use helper::remove_projectile_mod_from_targets_gun_system::remove_projectile_mod_from_targets_gun_system;

use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::events::remove_mod_from_target_event::RemoveModFromTargetEvent;
use crate::models::modifications::acid_puddle::{AcidPuddle, AcidPuddleOwner};
use crate::models::modifications::affects::affect_damage::AffectDamage;
use crate::models::modifications::affects::affect_health::AffectHealth;
use crate::models::modifications::affects::affect_hit_limit::AffectHitLimit;
use crate::models::modifications::affects::affect_move_speed::AffectMoveSpeed;
use crate::models::modifications::affects::affect_reload::AffectReload;
use crate::models::modifications::affects::affect_travel_range::AffectTravelRange;
use crate::models::modifications::affects::affect_unit_size::AffectUnitSize;
use crate::models::modifications::affects::projectile_affects::affect_projectile_damage::AffectProjectileDamage;
use crate::models::modifications::affects::projectile_affects::affect_projectile_hit_limit::AffectProjectileHitLimit;
use crate::models::modifications::affects::projectile_affects::affect_projectile_move_speed::AffectProjectileMoveSpeed;
use crate::models::modifications::affects::projectile_affects::affect_projectile_travel_range::AffectProjectileTravelRange;
use crate::models::modifications::affects::projectile_affects::affect_projectile_unit_size::AffectProjectileUnitSize;
use crate::models::modifications::burning_shot::BurningShot;
use crate::models::modifications::curve_shot::CurveShot;
use crate::models::modifications::death_ball::{DeathBall, DeathBallUnit};
use crate::models::modifications::explosion_shot::ExplosionShot;
use crate::models::modifications::gravity_shot::GravityShot;
use crate::models::modifications::grow_shot::GrowShot;
use crate::models::modifications::knock_back_shot::KnockBackShot;
use crate::models::modifications::lightning::Lightning;
use crate::models::modifications::magnet::Magnet;
use crate::models::modifications::psy_rock::{PsyRock, PsyRockUnit};
use crate::models::modifications::radiation::{Radiation, RadiationUnit};
use crate::models::modifications::shield::{Shield, ShieldUnit};
use crate::models::modifications::slime::{Slime, SlimeUnit};
use crate::models::modifications::split_shot::SplitShot;
use crate::models::modifications::sprinting::Sprinting;
use crate::models::modifications::turret::{Turret, TurretUnit};
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::health::Health;
use crate::models::unit_attributes::hit_limit::HitLimit;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::reload::Reload;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::models::unit_attributes::unit_size::UnitSize;
use crate::scheduling::BaseSets;
use crate::units::modifications::apply_acid_puddle_system::apply_acid_puddle_system;
use crate::units::modifications::apply_death_ball_system::apply_death_ball_system;
use crate::units::modifications::apply_psy_rock_system::{apply_psy_rock_system, renew_mods_for_psy_rock_system};
use crate::units::modifications::apply_radiation_system::apply_radiation_system;
use crate::units::modifications::apply_shield_system::apply_shield_system;
use crate::units::modifications::apply_slime_system::apply_slime_system;
use crate::units::modifications::apply_turret_system::apply_turret_system;
use crate::units::modifications::effect::apply_effect_add_health_system::apply_effect_add_health_system;
use crate::units::modifications::effect::apply_effect_damage_health_system::apply_effect_damage_health_system;
use crate::units::modifications::helper::apply_projectile_affect_system::apply_projectile_affect_system;
use crate::units::modifications::helper::despawn_companion_from_mod_system::despawn_companion_from_mod_system;
use crate::units::modifications::helper::mod_list_deregister_system::mod_list_deregister_system;
use crate::units::modifications::helper::mod_list_register_system::mod_list_register_system;
use crate::units::modifications::helper::remove_projectile_affect_system::remove_projectile_affect_system;
use crate::units::modifications::projectile_modifications::ProjectileModificationsPlugin;
use crate::units::modifications::statuse::StatusPlugin;

mod apply_turret_system;
mod apply_slime_system;
mod helper;
mod apply_death_ball_system;
mod apply_psy_rock_system;
mod apply_radiation_system;
mod apply_shield_system;
mod effect;
mod projectile_modifications;
mod apply_acid_puddle_system;
mod statuse;

/// All the apply systems have to get registered here.
///
/// The first [SystemSet] has the [RunCriteria] to run everything only if there is an
/// [ApplyModToTargetEvent].
/// Everything run in this set is related to applying a mod to a target.
///
/// The second [SystemSet] has the [RunCriteria] to run everything only if there is an
/// [RemoveModFromTargetEvent].
/// Everything run in this set is related to removing a mod from a target.
pub struct UnitModificationsPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ApplyUnitModificationsSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct RemoveUnitModificationsSystemSet;

impl Plugin for UnitModificationsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            ApplyUnitModificationsSystemSet
                .in_base_set(BaseSets::PostUpdate)
                .run_if(on_event::<ApplyModToTargetEvent>())
        );

        app.configure_set(
            RemoveUnitModificationsSystemSet
                .in_base_set(BaseSets::PostUpdate)
                .run_if(on_event::<RemoveModFromTargetEvent>())
        );

        app
            .add_plugin(ProjectileModificationsPlugin)
            .add_plugin(StatusPlugin)

            .add_system(mod_list_register_system.in_set(ApplyUnitModificationsSystemSet))

            .add_system(apply_affect_system::<MoveSpeed, AffectMoveSpeed>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_affect_system::<Damage, AffectDamage>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_affect_system::<Health, AffectHealth>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_affect_system::<Reload, AffectReload>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_affect_system::<UnitSize, AffectUnitSize>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_affect_system::<TravelRange, AffectTravelRange>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_affect_system::<HitLimit, AffectHitLimit>.in_set(ApplyUnitModificationsSystemSet))

            .add_system(apply_projectile_affect_system::<MoveSpeed, AffectProjectileMoveSpeed>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_projectile_affect_system::<Damage, AffectProjectileDamage>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_projectile_affect_system::<TravelRange, AffectProjectileTravelRange>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_projectile_affect_system::<HitLimit, AffectProjectileHitLimit>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_projectile_affect_system::<UnitSize, AffectProjectileUnitSize>.in_set(ApplyUnitModificationsSystemSet))

            .add_system(apply_effect_add_health_system.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_effect_damage_health_system.in_set(ApplyUnitModificationsSystemSet))

            .add_system(apply_projectile_mod_to_targets_gun_system::<CurveShot>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_projectile_mod_to_targets_gun_system::<GrowShot>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_projectile_mod_to_targets_gun_system::<SplitShot>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_projectile_mod_to_targets_gun_system::<GravityShot>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_projectile_mod_to_targets_gun_system::<KnockBackShot>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_projectile_mod_to_targets_gun_system::<ExplosionShot>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_projectile_mod_to_targets_gun_system::<Lightning>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_projectile_mod_to_targets_gun_system::<BurningShot>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_projectile_mod_to_targets_gun_system::<AcidPuddle>.in_set(ApplyUnitModificationsSystemSet))

            .add_system(apply_player_mod_to_target_system::<Sprinting>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_player_mod_to_target_system::<Turret>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_turret_system.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_player_mod_to_target_system::<Slime>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_slime_system.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_player_mod_to_target_system::<DeathBall>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_death_ball_system.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_player_mod_to_target_system::<PsyRock>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_psy_rock_system.in_set(ApplyUnitModificationsSystemSet))
            .add_system(renew_mods_for_psy_rock_system.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_player_mod_to_target_system::<Radiation>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_radiation_system.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_player_mod_to_target_system::<Shield>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_shield_system.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_player_mod_to_target_system::<Magnet>.in_set(ApplyUnitModificationsSystemSet))
            .add_system(apply_acid_puddle_system.in_set(ApplyUnitModificationsSystemSet));

        app
            .add_system(mod_list_deregister_system.in_set(RemoveUnitModificationsSystemSet))

            .add_system(remove_affect_system::<MoveSpeed, AffectMoveSpeed>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_affect_system::<Damage, AffectDamage>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_affect_system::<Health, AffectHealth>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_affect_system::<Reload, AffectReload>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_affect_system::<UnitSize, AffectUnitSize>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_affect_system::<TravelRange, AffectTravelRange>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_affect_system::<HitLimit, AffectHitLimit>.in_set(RemoveUnitModificationsSystemSet))

            .add_system(remove_projectile_affect_system::<MoveSpeed, AffectProjectileMoveSpeed>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_projectile_affect_system::<Damage, AffectProjectileDamage>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_projectile_affect_system::<TravelRange, AffectProjectileTravelRange>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_projectile_affect_system::<HitLimit, AffectProjectileHitLimit>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_projectile_affect_system::<UnitSize, AffectProjectileUnitSize>.in_set(RemoveUnitModificationsSystemSet))

            .add_system(remove_projectile_mod_from_targets_gun_system::<CurveShot>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_projectile_mod_from_targets_gun_system::<GrowShot>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_projectile_mod_from_targets_gun_system::<SplitShot>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_projectile_mod_from_targets_gun_system::<GravityShot>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_projectile_mod_from_targets_gun_system::<KnockBackShot>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_projectile_mod_from_targets_gun_system::<ExplosionShot>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_projectile_mod_from_targets_gun_system::<Lightning>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_projectile_mod_from_targets_gun_system::<AcidPuddle>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_projectile_mod_from_targets_gun_system::<BurningShot>.in_set(RemoveUnitModificationsSystemSet))

            .add_system(remove_player_mod_from_target_system::<Sprinting>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_player_mod_from_target_system::<Turret>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(despawn_companion_from_mod_system::<Turret, TurretUnit>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_player_mod_from_target_system::<Slime>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(despawn_companion_from_mod_system::<Slime, SlimeUnit>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_player_mod_from_target_system::<DeathBall>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(despawn_companion_from_mod_system::<DeathBall, DeathBallUnit>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_player_mod_from_target_system::<PsyRock>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(despawn_companion_from_mod_system::<PsyRock, PsyRockUnit>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_player_mod_from_target_system::<Radiation>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(despawn_companion_from_mod_system::<Radiation, RadiationUnit>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_player_mod_from_target_system::<Shield>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(despawn_companion_from_mod_system::<Shield, ShieldUnit>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(despawn_companion_from_mod_system::<AcidPuddle, AcidPuddleOwner>.in_set(RemoveUnitModificationsSystemSet))
            .add_system(remove_player_mod_from_target_system::<Magnet>.in_set(RemoveUnitModificationsSystemSet));
    }
}

