use bevy::prelude::ResMut;

use crate::models::configurations::enemy_config::EnemyConfig;
use crate::util::read_file_to_string::read_file_to_string;

#[derive(Default)]
pub struct EnemyConfigHandles {
    pub goblin: EnemyConfig,
}

pub fn preload_enemy_system(
    mut enemy_handles: ResMut<EnemyConfigHandles>,
) {
    let my_string = read_file_to_string("configurations/goblin.json");
    enemy_handles.goblin = serde_json::from_str(&my_string).expect("JSON was not well-formatted");
}

