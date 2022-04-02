use std::fs::read_to_string;
use bevy::prelude::{Handle, ResMut};
use serde::{Serialize, Deserialize};
use crate::assets_handling::configurations::enemy_config::EnemyConfig;
use crate::components::unit_stats_components::Enemy;

#[derive(Default)]
pub struct EnemyConfigHandles {
    pub rock: EnemyConfig,
}

pub fn preload_enemy_system(
    mut enemy_handles: ResMut<EnemyConfigHandles>,
){
    let mut my_string = match read_to_string("assets/configurations/Rock.json") {
        Ok(value) => value,
        Err(_) => String::new(),
    };

    enemy_handles.rock = serde_json::from_str(&my_string).expect("JSON was not well-formatted");
}

