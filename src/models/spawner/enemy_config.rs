use bevy::prelude::Vec2;
use serde::Deserialize;

use crate::SpriteLayer;

#[derive(Default, Deserialize)]
pub struct EnemyConfig {
    pub config_id: usize,
    pub entity_name: String,

    pub size: Vec2,
    pub sprite_path: String,
    pub sprite_layer: SpriteLayer,

    pub collider_weight: f32,

    pub base_damage: f32,
    pub damage_interval: f32,

    pub move_speed: f32,
    pub health: f32,
}