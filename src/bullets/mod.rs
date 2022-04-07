use bevy::prelude::{CoreStage, Plugin};

use crate::App;
use crate::bullets::bullet_despawn_system::bullet_despawn_system;
use crate::bullets::bullet_hit_system::bullet_hit_system;
use crate::bullets::bullet_movement_system::bullet_movement_system;
use crate::bullets::modifications::ModificationsPlugin;

pub mod bullet_movement_system;
pub mod modifications;
pub mod bullet_hit_system;
pub mod bullet_despawn_system;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ModificationsPlugin)

            .add_system(bullet_movement_system)

            .add_system_to_stage(CoreStage::PreUpdate, bullet_hit_system)
            .add_system_to_stage(CoreStage::Last, bullet_despawn_system);
    }
}