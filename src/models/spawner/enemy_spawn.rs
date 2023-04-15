use serde::Deserialize;

#[derive(Default, Deserialize, Clone)]
pub struct EnemySpawn {
    pub enemy_index: usize,
    pub spawn_weight: f32,
}