use bevy::prelude::ResMut;

use crate::assets_handling::configurations::coin_config::CoinConfig;
use crate::assets_handling::configurations::hot_dog_config::HotDogConfig;
use crate::assets_handling::configurations::kiste_config::KisteConfig;
use crate::util::read_file_to_string::read_file_to_string;

#[derive(Default)]
pub struct ItemConfigHandles {
    pub coin: CoinConfig,
    pub hot_dog: HotDogConfig,
    pub kiste: KisteConfig
}

pub fn preload_item_system(
    mut item_config_handles: ResMut<ItemConfigHandles>,
) {
    let my_string = read_file_to_string("configurations/coin.json");
    item_config_handles.coin = serde_json::from_str(&my_string).expect("JSON was not well-formatted || COIN");

    let my_string = read_file_to_string("configurations/hot_dog.json");
    item_config_handles.hot_dog = serde_json::from_str(&my_string).expect("JSON was not well-formatted || HOT DOG");

    let my_string = read_file_to_string("configurations/kiste.json");
    item_config_handles.kiste = serde_json::from_str(&my_string).expect("JSON was not well-formatted || KISTE");
}