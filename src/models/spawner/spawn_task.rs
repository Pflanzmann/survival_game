use bevy::prelude::{Entity, Vec2};

pub struct SpawnTask {
    pub enemy_config_index: usize,
    pub position: Vec2,
    pub target_player: Entity,
}

impl SpawnTask {
    pub fn new(enemy_config_index: usize, position: Vec2, target_player: Entity) -> Self {
        SpawnTask { enemy_config_index, position, target_player }
    }
}
