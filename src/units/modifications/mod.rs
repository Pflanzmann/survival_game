use bevy::app::Plugin;
use bevy::prelude::{App, SystemSet};

use helper::apply_affect_system::apply_affect_system;
use helper::apply_bullet_mod_to_targets_gun_system::apply_bullet_mod_to_targets_gun_system;
use helper::apply_player_mod_to_target_system::apply_player_mod_to_target_system;
use helper::remove_affect_system::remove_affect_system;
use helper::remove_bullet_mod_from_targets_gun_system::remove_bullet_mod_from_targets_gun_system;
use helper::remove_player_mod_from_target_system::remove_player_mod_from_target_system;

use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::events::remove_mod_from_target_event::RemoveModFromTargetEvent;
use crate::models::modifications::affects::affect_damage::AffectDamage;
use crate::models::modifications::affects::affect_health::AffectHealth;
use crate::models::modifications::affects::affect_hit_limit::AffectHitLimit;
use crate::models::modifications::affects::affect_move_speed::AffectMoveSpeed;
use crate::models::modifications::affects::affect_reload::AffectReload;
use crate::models::modifications::affects::affect_travel_range::AffectTravelRange;
use crate::models::modifications::affects::affect_unit_size::AffectUnitSize;
use crate::models::modifications::affects::bullet_affects::affect_bullet_damage::AffectBulletDamage;
use crate::models::modifications::affects::bullet_affects::affect_bullet_hit_limit::AffectBulletHitLimit;
use crate::models::modifications::affects::bullet_affects::affect_bullet_move_speed::AffectBulletMoveSpeed;
use crate::models::modifications::affects::bullet_affects::affect_bullet_travel_range::AffectBulletTravelRange;
use crate::models::modifications::affects::bullet_affects::affect_bullet_unit_size::AffectBulletUnitSize;
use crate::models::modifications::curve_shot::CurveShot;
use crate::models::modifications::death_ball::{DeathBall, DeathBallUnit};
use crate::models::modifications::explosion_shot::ExplosionShot;
use crate::models::modifications::gravity_shot::GravityShot;
use crate::models::modifications::grow_shot::GrowShot;
use crate::models::modifications::knock_back_shot::KnockBackShot;
use crate::models::modifications::lightning::Lightning;
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
use crate::units::modifications::apply_death_ball_system::apply_death_ball_system;
use crate::units::modifications::apply_psy_rock_system::{apply_psy_rock_system, renew_mods_for_psy_rock_system};
use crate::units::modifications::apply_radiation_system::apply_radiation_system;
use crate::units::modifications::apply_shield_system::apply_shield_system;
use crate::units::modifications::apply_slime_system::apply_slime_system;
use crate::units::modifications::apply_turret_system::apply_turret_system;
use crate::units::modifications::effect::apply_effect_add_health_system::apply_effect_add_health_system;
use crate::units::modifications::effect::apply_effect_damage_health_system::apply_effect_damage_health_system;
use crate::units::modifications::helper::apply_bullet_affect_system::apply_bullet_affect_system;
use crate::units::modifications::helper::despawn_companion_from_mod_system::despawn_companion_from_mod_system;
use crate::units::modifications::helper::mod_list_deregister_system::mod_list_deregister_system;
use crate::units::modifications::helper::mod_list_register_system::mod_list_register_system;
use crate::units::modifications::helper::remove_bullet_affect_system::remove_bullet_affect_system;
use crate::util::run_criteria::on_event::on_event;
use crate::util::stage_label_helper::in_post_update;

mod apply_turret_system;
mod apply_slime_system;
mod helper;
mod apply_death_ball_system;
mod apply_psy_rock_system;
mod apply_radiation_system;
mod apply_shield_system;
mod effect;
pub mod bullet_modifications;

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

impl Plugin for UnitModificationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_post_update(
                    SystemSet::new()
                        .with_run_criteria(on_event::<ApplyModToTargetEvent>)

                        .with_system(mod_list_register_system)

                        .with_system(apply_affect_system::<MoveSpeed, AffectMoveSpeed>)
                        .with_system(apply_affect_system::<Damage, AffectDamage>)
                        .with_system(apply_affect_system::<Health, AffectHealth>)
                        .with_system(apply_affect_system::<Reload, AffectReload>)
                        .with_system(apply_affect_system::<UnitSize, AffectUnitSize>)
                        .with_system(apply_affect_system::<TravelRange, AffectTravelRange>)
                        .with_system(apply_affect_system::<HitLimit, AffectHitLimit>)

                        .with_system(apply_bullet_affect_system::<MoveSpeed, AffectBulletMoveSpeed>)
                        .with_system(apply_bullet_affect_system::<Damage, AffectBulletDamage>)
                        .with_system(apply_bullet_affect_system::<TravelRange, AffectBulletTravelRange>)
                        .with_system(apply_bullet_affect_system::<HitLimit, AffectBulletHitLimit>)
                        .with_system(apply_bullet_affect_system::<UnitSize, AffectBulletUnitSize>)

                        .with_system(apply_effect_add_health_system)
                        .with_system(apply_effect_damage_health_system)

                        .with_system(apply_bullet_mod_to_targets_gun_system::<CurveShot>)
                        .with_system(apply_bullet_mod_to_targets_gun_system::<GrowShot>)
                        .with_system(apply_bullet_mod_to_targets_gun_system::<SplitShot>)
                        .with_system(apply_bullet_mod_to_targets_gun_system::<GravityShot>)
                        .with_system(apply_bullet_mod_to_targets_gun_system::<KnockBackShot>)
                        .with_system(apply_bullet_mod_to_targets_gun_system::<ExplosionShot>)
                        .with_system(apply_bullet_mod_to_targets_gun_system::<Lightning>)

                        .with_system(apply_player_mod_to_target_system::<Sprinting>)

                        .with_system(apply_player_mod_to_target_system::<Turret>)
                        .with_system(apply_turret_system)

                        .with_system(apply_player_mod_to_target_system::<Slime>)
                        .with_system(apply_slime_system)

                        .with_system(apply_player_mod_to_target_system::<DeathBall>)
                        .with_system(apply_death_ball_system)

                        .with_system(apply_player_mod_to_target_system::<PsyRock>)
                        .with_system(apply_psy_rock_system)
                        .with_system(renew_mods_for_psy_rock_system)

                        .with_system(apply_player_mod_to_target_system::<Radiation>)
                        .with_system(apply_radiation_system)

                        .with_system(apply_player_mod_to_target_system::<Shield>)
                        .with_system(apply_shield_system)
                )
            )

            .add_system_set(
                in_post_update(
                    SystemSet::new()
                        .with_run_criteria(on_event::<RemoveModFromTargetEvent>)

                        .with_system(mod_list_deregister_system)

                        .with_system(remove_affect_system::<MoveSpeed, AffectMoveSpeed>)
                        .with_system(remove_affect_system::<Damage, AffectDamage>)
                        .with_system(remove_affect_system::<Health, AffectHealth>)
                        .with_system(remove_affect_system::<Reload, AffectReload>)
                        .with_system(remove_affect_system::<UnitSize, AffectUnitSize>)
                        .with_system(remove_affect_system::<TravelRange, AffectTravelRange>)
                        .with_system(remove_affect_system::<HitLimit, AffectHitLimit>)

                        .with_system(remove_bullet_affect_system::<MoveSpeed, AffectBulletMoveSpeed>)
                        .with_system(remove_bullet_affect_system::<Damage, AffectBulletDamage>)
                        .with_system(remove_bullet_affect_system::<TravelRange, AffectBulletTravelRange>)
                        .with_system(remove_bullet_affect_system::<HitLimit, AffectBulletHitLimit>)
                        .with_system(remove_bullet_affect_system::<UnitSize, AffectBulletUnitSize>)

                        .with_system(remove_bullet_mod_from_targets_gun_system::<CurveShot>)
                        .with_system(remove_bullet_mod_from_targets_gun_system::<GrowShot>)
                        .with_system(remove_bullet_mod_from_targets_gun_system::<SplitShot>)
                        .with_system(remove_bullet_mod_from_targets_gun_system::<GravityShot>)
                        .with_system(remove_bullet_mod_from_targets_gun_system::<KnockBackShot>)
                        .with_system(remove_bullet_mod_from_targets_gun_system::<ExplosionShot>)
                        .with_system(remove_bullet_mod_from_targets_gun_system::<Lightning>)

                        .with_system(remove_player_mod_from_target_system::<Sprinting>)

                        .with_system(remove_player_mod_from_target_system::<Turret>)
                        .with_system(despawn_companion_from_mod_system::<Turret, TurretUnit>)

                        .with_system(remove_player_mod_from_target_system::<Slime>)
                        .with_system(despawn_companion_from_mod_system::<Slime, SlimeUnit>)

                        .with_system(remove_player_mod_from_target_system::<DeathBall>)
                        .with_system(despawn_companion_from_mod_system::<DeathBall, DeathBallUnit>)

                        .with_system(remove_player_mod_from_target_system::<PsyRock>)
                        .with_system(despawn_companion_from_mod_system::<PsyRock, PsyRockUnit>)

                        .with_system(remove_player_mod_from_target_system::<Radiation>)
                        .with_system(despawn_companion_from_mod_system::<Radiation, RadiationUnit>)

                        .with_system(remove_player_mod_from_target_system::<Shield>)
                        .with_system(despawn_companion_from_mod_system::<Shield, ShieldUnit>)
                )
            )
        ;
    }
}

