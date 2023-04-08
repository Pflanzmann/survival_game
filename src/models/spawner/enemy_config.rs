use bevy::prelude::{Handle, TextureAtlas, Vec2};
use crate::models::configurations::raw_configs::enemy_behavior::EnemyBehavior;

use crate::SpriteLayer;

pub struct EnemyConfig {
    pub config_id: usize,
    pub entity_name: String,

    pub size: Vec2,
    pub texture_atlas: Handle<TextureAtlas>,
    pub sprite_layer: SpriteLayer,

    pub collider_weight: f32,

    pub base_damage: f32,
    pub damage_interval: f32,

    pub move_speed: f32,
    pub health: f32,

    pub behavior: EnemyBehavior,
}
