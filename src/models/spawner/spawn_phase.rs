use serde::Deserialize;

use crate::models::spawner::enemy_spawn::EnemySpawn;
use crate::models::spawner::spawn_pattern::SpawnPattern;

#[derive(Default, Deserialize)]
pub struct SpawnPhase {
    #[serde(default)]
    pub duration: f32,
    #[serde(default)]
    pub spawn_interval: f32,
    #[serde(default)]
    pub minimum_enemy_amount: usize,
    pub enemies: Vec<EnemySpawn>,
    #[serde(default)]
    pub pattern: SpawnPattern,
}
