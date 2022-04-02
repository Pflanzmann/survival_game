use serde::{Serialize, Deserialize};

#[derive(Default, Deserialize, Debug)]
pub struct EnemyConfig {
    pub sprite_custom_size_x: f32,
    pub sprite_custom_size_y: f32,
    pub sprite_path: String,
    pub move_speed: f32,
    pub damage: f32,
    pub health: f32,
}