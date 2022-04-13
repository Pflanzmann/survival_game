use bevy::prelude::ResMut;
use crate::models::configurations::barrel_config::BarrelConfig;

use crate::models::configurations::coin_config::CoinConfig;
use crate::models::configurations::hot_dog_config::HotDogConfig;

use crate::util::read_file_to_string::read_file_to_string;

#[derive(Default)]
pub struct ItemConfigHandles {
    pub coin: CoinConfig,
    pub hot_dog: HotDogConfig,
    pub barrel: BarrelConfig,
}

pub fn preload_item_system(
    mut item_config_handles: ResMut<ItemConfigHandles>,
) {
    let my_string = read_file_to_string("configurations/coin.json");
    item_config_handles.coin = serde_json::from_str(&my_string).expect("JSON was not well-formatted || COIN");

    let my_string = read_file_to_string("configurations/hot_dog.json");
    item_config_handles.hot_dog = serde_json::from_str(&my_string).expect("JSON was not well-formatted || HOT DOG");

    let my_string = read_file_to_string("configurations/barrel.json");
    item_config_handles.barrel = serde_json::from_str(&my_string).expect("JSON was not well-formatted || Barrel");
}