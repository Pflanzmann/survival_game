use bevy::prelude::Commands;

use crate::models::configurations::world_grid_config::WorldGridConfig;
use crate::util::read_file_to_string::read_file_to_string;

pub fn preload_world_grid_config_system(
    mut commands: Commands,
) {
    let my_string = read_file_to_string("configurations/world_grid.json");
    let world_grid_config: WorldGridConfig = serde_json::from_str(&my_string).expect("JSON was not well-formatted || BASIC_BULLET");

    commands.insert_resource(world_grid_config);
}