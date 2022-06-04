use bevy::prelude::{Plugin, SystemSet};

use crate::{App, AppState};
use crate::units::projectile::projectile_check_stop_system::projectile_check_stop_system;
use crate::units::projectile::projectile_despawn_system::projectile_despawn_system;
use crate::util::stage_label_helper::{in_last, in_update};

mod projectile_check_stop_system;
mod projectile_despawn_system;

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
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(projectile_check_stop_system)
                )
            )

            .add_system_set(
                in_last(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(projectile_despawn_system)
                )
            );
    }
}