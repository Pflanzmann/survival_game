use bevy::prelude::{App, Plugin};

use crate::{AppState, CoreStage, SystemSet};
use crate::drops::basic_drop_system::basic_drop_system;
use crate::drops::coin_pickup_system::coin_pickup_system;
use crate::drops::hot_dog_pickup_system::hot_dog_pickup_system;

pub mod basic_drop_system;
pub mod coin_pickup_system;
pub mod hot_dog_pickup_system;

pub struct DropsPlugin;

impl Plugin for DropsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(AppState::InGame)
                .with_system(basic_drop_system)
            )
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .with_system(coin_pickup_system)
                    .with_system(hot_dog_pickup_system),
            );
    }
}