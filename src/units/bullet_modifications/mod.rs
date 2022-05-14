use bevy::prelude::{Plugin, SystemSet};

use crate::{App, AppState};
use crate::models::events::bullet_shot_event::BulletShotEvent;
use crate::models::modifications::curve_shot::CurveShot;
use crate::models::modifications::explosion_shot::ExplosionShot;
use crate::models::modifications::gravity_shot::GravityShot;
use crate::models::modifications::grow_shot::GrowShot;
use crate::models::modifications::knock_back_shot::KnockBackShot;
use crate::models::modifications::split_shot::SplitShot;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::hit_limit::HitLimit;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::units::bullet_modifications::assign_attribute_to_bullet_system::assign_attribute_to_bullet_system;
use crate::units::bullet_modifications::assign_modification_to_bullet_system::assign_modification_to_bullet_system;
use crate::units::bullet_modifications::curve_shot_system::curve_shot_system;
use crate::units::bullet_modifications::explosion_shot_system::explosion_shot_system;
use crate::units::bullet_modifications::gravity_shot::gravity_shot_system;
use crate::units::bullet_modifications::grow_shot_system::grow_shot_system;
use crate::units::bullet_modifications::knock_back_shot_system::knock_back_shot_system;
use crate::units::bullet_modifications::split_shot_system::split_shot_system;
use crate::util::run_criteria::on_event::on_event;
use crate::util::stage_label_helper::in_post_update;

mod curve_shot_system;
mod grow_shot_system;
mod split_shot_system;
mod assign_modification_to_bullet_system;
mod assign_attribute_to_bullet_system;
mod gravity_shot;
mod knock_back_shot_system;
mod explosion_shot_system;

/// This plugin manages the [Bullet]-[Modification]s and how they get applied.
/// All the systems get added in the [PreUpdate] so that they can react last to all
/// other accesses of the bullet.
///
/// All system get only used in the [AppState::Ingame].
pub struct BulletModificationsPlugin;

impl Plugin for BulletModificationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_post_update(
                    SystemSet::new()
                        .with_run_criteria(on_event::<BulletShotEvent>)

                        .with_system(assign_attribute_to_bullet_system::<Damage>)
                        .with_system(assign_attribute_to_bullet_system::<HitLimit>)
                        .with_system(assign_attribute_to_bullet_system::<MoveSpeed>)
                        .with_system(assign_attribute_to_bullet_system::<TravelRange>)

                        .with_system(assign_modification_to_bullet_system::<CurveShot>)
                        .with_system(assign_modification_to_bullet_system::<GrowShot>)
                        .with_system(assign_modification_to_bullet_system::<SplitShot>)
                        .with_system(assign_modification_to_bullet_system::<GravityShot>)
                        .with_system(assign_modification_to_bullet_system::<KnockBackShot>)
                        .with_system(assign_modification_to_bullet_system::<ExplosionShot>)
                )
            )

            .add_system_set(
                in_post_update(
                    SystemSet::on_update(AppState::InGame)

                        .with_system(curve_shot_system)
                        .with_system(grow_shot_system)
                        .with_system(split_shot_system)
                        .with_system(gravity_shot_system)
                        .with_system(knock_back_shot_system)
                        .with_system(explosion_shot_system)
                )
            );
    }
}
