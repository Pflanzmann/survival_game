use bevy::prelude::{Plugin, SystemSet};

use crate::{App, AppState};
use crate::units::bullet_modifications::BulletModificationsPlugin;
use crate::units::bullets::bullet_check_stop_system::bullet_check_stop_system;
use crate::units::bullets::bullet_despawn_system::bullet_despawn_system;
use crate::units::bullets::bullet_hit_system::bullet_hit_system;
use crate::util::stage_label_helper::{in_last, in_pre_update, in_update};

mod bullet_check_stop_system;
mod bullet_hit_system;
mod bullet_despawn_system;

/// This plugin manages the [Bullet] systems and how they get applied.
///
/// [bullet_hit_system] gets run in the [on_pre_update] stack because it is a system that
/// reacts directly to the collision systems
///
/// [bullet_check_stop_system] gets run in the [on_update] stack
///
/// [bullet_despawn_system] gets run in the [on_last] stack because the app panics if
/// you try access a despawned entity
///
/// All system get only used in the [AppState::InGame].
pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(BulletModificationsPlugin)

            .add_system_set(
                in_pre_update(SystemSet::on_update(AppState::InGame)
                    .with_system(bullet_hit_system)
                )
            )

            .add_system_set(
                in_update(SystemSet::on_update(AppState::InGame)
                    .with_system(bullet_check_stop_system)
                )
            )

            .add_system_set(
                in_last(SystemSet::on_update(AppState::InGame)
                    .with_system(bullet_despawn_system)
                )
            );
    }
}