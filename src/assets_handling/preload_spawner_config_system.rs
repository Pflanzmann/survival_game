use bevy::prelude::Commands;
use crate::models::configurations::spawner_config::SpawnerConfig;

use crate::util::read_file_to_string::read_file_to_string;

pub fn preload_spawner_config_system(
    mut commands: Commands,
) {
    let my_string = read_file_to_string("configurations/spawner_config.json");
    let spawner_config: SpawnerConfig = serde_json::from_str(&my_string).expect("JSON was not well-formatted");

    commands.insert_resource(spawner_config);
}
