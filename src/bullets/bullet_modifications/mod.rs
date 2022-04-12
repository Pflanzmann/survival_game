use bevy::ecs::schedule::StageLabel;
use bevy::prelude::{Plugin, SystemSet};

use crate::{App, AppState};
use crate::bullets::bullet_modifications::apply_modification_system::apply_modification_system;
use crate::bullets::bullet_modifications::curve_shot_system::curve_shot_system;
use crate::bullets::bullet_modifications::grow_shot_system::grow_shot_system;
use crate::bullets::bullet_modifications::split_shot_system::split_shot_system;
use crate::models::modifications::grow_shot::GrowShot;
use crate::models::modifications::curve_shot::CurveShot;
use crate::models::modifications::split_shot::SplitShot;
use crate::util::stage_label_helper::in_pre_update;

mod curve_shot_system;
mod grow_shot_system;
mod split_shot_system;
mod apply_modification_system;

/// This plugin manages the [`Bullet-Modification`]s and how they get applied.
/// All the systems get added in the [PreUpdate] so that they can react last to all
/// other accesses of the bullet.
///
/// All system get only used in the [`AppState::Ingame`].
pub struct BulletModificationsPlugin;

impl Plugin for BulletModificationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_pre_update(
                    SystemSet::on_update(AppState::InGame)

                        .with_system(apply_modification_system::<CurveShot>)
                        .with_system(curve_shot_system)

                        .with_system(apply_modification_system::<GrowShot>)
                        .with_system(grow_shot_system)

                        .with_system(apply_modification_system::<SplitShot>)
                        .with_system(split_shot_system)
                )
            );
    }
}
