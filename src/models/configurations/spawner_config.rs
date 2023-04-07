use bevy::prelude::Resource;
use serde::Deserialize;

#[derive(Default, Resource, Deserialize)]
pub struct SpawnerConfig {
    pub spawn_range: f32,
    pub spawn_range_offset: f32,
    pub max_spawns_per_frame: usize,
}