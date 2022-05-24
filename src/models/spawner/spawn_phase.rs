use serde::Deserialize;

use crate::models::spawner::enemy_spawn::EnemySpawn;

#[derive(Default, Deserialize)]
pub struct SpawnPhase {
    pub duration: f32,
    pub spawn_interval: f32,
    pub enemies: Vec<EnemySpawn>,
}
