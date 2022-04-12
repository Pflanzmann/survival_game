use bevy::prelude::{App, Plugin};

use crate::{AppState, SystemSet};
use crate::drops::basic_drop_system::basic_drop_system;
use crate::drops::coin_pickup_system::coin_pickup_system;
use crate::drops::hot_dog_pickup_system::hot_dog_pickup_system;
use crate::drops::kiste_pickup_system::kiste_pickup_system;
use crate::util::stage_label_helper::{in_pre_update, in_update};

mod basic_drop_system;
mod coin_pickup_system;
mod hot_dog_pickup_system;
mod kiste_pickup_system;

pub struct DropsPlugin;

impl Plugin for DropsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(basic_drop_system)
                )
            )
            .add_system_set(
                in_pre_update(
                    SystemSet::new()
                        .with_system(coin_pickup_system)
                        .with_system(hot_dog_pickup_system)
                        .with_system(kiste_pickup_system),
                )
            );
    }
}