use bevy::prelude::{Assets, AssetServer, Res, ResMut, TextureAtlas, Vec2};
use serde::Deserialize;

use crate::models::configurations::raw_configs::enemy_behavior::EnemyBehavior;
use crate::models::spawner::enemy_config::EnemyConfig;
use crate::SpriteLayer;

#[derive(Default, Deserialize)]
pub struct RawEnemyConfig {
    pub config_id: usize,
    pub entity_name: String,

    pub size: Vec2,
    pub sprite_path: String,
    pub texture_atlas_grid_size: Vec2,
    pub texture_atlas_columns: usize,
    pub texture_atlas_rows: usize,
    pub sprite_layer: SpriteLayer,

    pub collider_weight: f32,

    pub base_damage: f32,
    pub damage_interval: f32,

    pub move_speed: f32,
    pub health: f32,

    #[serde(default)]
    pub behavior: EnemyBehavior,
}

impl RawEnemyConfig {
    pub fn get_config(
        &self,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> EnemyConfig {
        EnemyConfig {
            config_id: self.config_id,
            entity_name: self.entity_name.clone(),
            size: self.size,
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load(&self.sprite_path), self.texture_atlas_grid_size, self.texture_atlas_columns, self.texture_atlas_rows, None, None)),
            sprite_layer: self.sprite_layer,
            collider_weight: self.collider_weight,
            base_damage: self.base_damage,
            damage_interval: self.damage_interval,
            move_speed: self.move_speed,
            health: self.health,
            behavior: self.behavior
        }
    }
}
