use bevy::app::IntoSystemAppConfig;
use bevy::prelude::*;

use crate::AppState;
use crate::scheduling::BaseSets;
use crate::world::drops::coin_pickup_system::coin_pickup_system;
use crate::world::drops::drop_chance_system::drop_chance_system;
use crate::world::drops::enter_shop_system::enter_shop_system;
use crate::world::drops::hot_dog_pickup_system::hot_dog_pickup_system;
use crate::world::drops::setup_shop_system::setup_shop_system;
use crate::world::drops::visited_shop_system::visited_shop_system;

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

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DropsEnterShopSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DropsUpdateSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DropsPreUpdateSystemSet;

impl Plugin for DropsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            DropsEnterShopSystemSet
                .in_base_set(BaseSets::Update)
                // .in_schedule(OnEnter(AppState::InGame))
        );

        app.configure_set(
            DropsUpdateSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(in_state(AppState::Shop))
        );

        app.configure_set(
            DropsPreUpdateSystemSet
                .in_base_set(BaseSets::PreUpdate)
                .run_if(in_state(AppState::InGame))
        );

        app.add_system(setup_shop_system.in_set(DropsEnterShopSystemSet).in_schedule(OnEnter(AppState::InGame)));

        app
            .add_system(drop_chance_system.in_set(DropsUpdateSystemSet))
            .add_system(visited_shop_system.in_set(DropsUpdateSystemSet));


        app
            .add_system(coin_pickup_system.in_set(DropsPreUpdateSystemSet))
            .add_system(hot_dog_pickup_system.in_set(DropsPreUpdateSystemSet))
            .add_system(enter_shop_system.in_set(DropsPreUpdateSystemSet));
    }
}
