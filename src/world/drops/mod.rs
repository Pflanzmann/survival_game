use bevy::prelude::{App, Plugin, SystemSet};

use crate::AppState;
use crate::world::drops::enter_shop_system::enter_shop_system;
use crate::world::drops::drop_chance_system::drop_chance_system;
use crate::world::drops::coin_pickup_system::coin_pickup_system;
use crate::world::drops::hot_dog_pickup_system::hot_dog_pickup_system;
use crate::world::drops::setup_shop_system::setup_shop_system;
use crate::world::drops::visited_shop_system::visited_shop_system;
use crate::util::stage_label_helper::{in_pre_update, in_update};

mod drop_chance_system;
mod coin_pickup_system;
mod hot_dog_pickup_system;
mod enter_shop_system;
mod setup_shop_system;
mod visited_shop_system;

/// this plugin manages the spawning and collection of items during the game.
///
/// [ basic_drop_system ] controls probability and collection of possible drops
/// whenever an enemy dies
///
/// [ hot_dog_pickup_system ], [ coin_pickup_system ] and [ barrel_pickup_system ]
/// handle the event when an item is picked up and execute the responding action.
pub struct DropsPlugin;

impl Plugin for DropsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
                    .with_system(setup_shop_system)
            )

            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(drop_chance_system)
                        .with_system(visited_shop_system)
                )
            )
            .add_system_set(
                in_pre_update(
                    SystemSet::new()
                        .with_system(coin_pickup_system)
                        .with_system(hot_dog_pickup_system)
                        .with_system(enter_shop_system),
                )
            );
    }
}