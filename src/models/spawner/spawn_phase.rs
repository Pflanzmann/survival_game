use serde::Deserialize;

use crate::models::spawner::enemy_spawn::EnemySpawn;
use crate::models::spawner::spawn_pattern::SpawnPattern;

#[derive(Default, Deserialize)]
pub struct SpawnPhase {
    pub duration: f32,
    pub spawn_interval: f32,
    pub minimum_enemy_amount: usize,
    pub enemies: Vec<EnemySpawn>,
    pub pattern: SpawnPattern
}
