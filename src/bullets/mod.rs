use bevy::prelude::{CoreStage, Plugin, SystemSet};

use crate::{App, AppState};
use crate::bullets::bullet_despawn_system::bullet_despawn_system;
use crate::bullets::bullet_hit_system::bullet_hit_system;
use crate::bullets::bullet_movement_system::bullet_movement_system;
use crate::bullets::modifications::ModificationsPlugin;
use crate::util::stage_label_helper::{in_last, in_pre_update, in_update};

pub mod bullet_movement_system;
pub mod modifications;
pub mod bullet_hit_system;
pub mod bullet_despawn_system;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ModificationsPlugin)

            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(bullet_movement_system)
                )
            )

            .add_system_set(
                in_pre_update(SystemSet::new()
                    .with_system(bullet_hit_system)
                )
            )

            .add_system_set(
                in_last(SystemSet::on_update(AppState::InGame)
                    .with_system(bullet_despawn_system)
                )
            );
    }
}