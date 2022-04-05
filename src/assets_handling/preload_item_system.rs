use bevy::prelude::ResMut;

use crate::assets_handling::configurations::item_config::ItemConfig;
use crate::util::read_file_to_string::read_file_to_string;

#[derive(Default)]
pub struct ItemConfigHandles {
    pub coin: ItemConfig,
}

pub fn preload_item_system(
    mut item_config_handles: ResMut<ItemConfigHandles>,
) {
    let my_string = read_file_to_string("configurations/coin.json");
    item_config_handles.coin = serde_json::from_str(&my_string).expect("JSON was not well-formatted");
}