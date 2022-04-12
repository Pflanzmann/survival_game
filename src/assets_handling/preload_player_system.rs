use bevy::prelude::ResMut;

use crate::assets_handling::configurations::player_config::PlayerConfig;
use crate::util::read_file_to_string::read_file_to_string;

#[derive(Default)]
pub struct PlayerConfigHandles {
    pub player_one: PlayerConfig,
}

pub fn preload_player_system(
    mut player_handles: ResMut<PlayerConfigHandles>,
) {
    let my_string = read_file_to_string("configurations/player.json");
    player_handles.player_one = serde_json::from_str(&my_string).expect("JSON was not well-formatted");
}