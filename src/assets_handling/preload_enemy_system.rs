use bevy::prelude::{Handle, ResMut};
use serde::{Serialize, Deserialize};
use crate::assets_handling::configurations::enemy_config::EnemyConfig;
use crate::components::unit_stats_components::Enemy;
use crate::util::read_file_to_string::read_file_to_string;

#[derive(Default)]
pub struct EnemyConfigHandles {
    pub rock: EnemyConfig,
}

pub fn preload_enemy_system(
    mut enemy_handles: ResMut<EnemyConfigHandles>,
){
    let my_string = read_file_to_string("assets/configurations/Rock.json");
    enemy_handles.rock = serde_json::from_str(&my_string).expect("JSON was not well-formatted");
}

