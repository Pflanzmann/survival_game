use bevy::prelude::Resource;

use crate::models::spawner::enemy_config::EnemyConfig;

#[derive(Default, Resource)]
pub struct EnemyConfigHandles {
    pub enemy_configs: Vec<EnemyConfig>,
}
