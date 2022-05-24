use std::fs;

use bevy::prelude::ResMut;

use crate::models::spawner::enemy_config::EnemyConfig;
use crate::models::spawner::enemy_config_handle::EnemyConfigHandles;
use crate::util::read_file_to_string::read_file_to_string;

pub fn preload_enemy_system(
    mut enemy_handles_new: ResMut<EnemyConfigHandles>,
) {
    let base_path = "configurations/enemies/";
    let paths = fs::read_dir(base_path).unwrap();

    for path in paths {
        let mut file_path = String::new();
        file_path.push_str(base_path);

        let my_string = read_file_to_string(&path.unwrap().path().display().to_string());

        let config: EnemyConfig = serde_json::from_str(&my_string).expect("JSON was not well-formatted");
        enemy_handles_new.enemy_configs.push(config);
    }
}

