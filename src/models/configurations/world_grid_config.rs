use bevy::prelude::Resource;
use serde::Deserialize;

#[derive(Default, Deserialize, Debug, Resource)]
pub struct WorldGridConfig {
    pub generated_background_radius_x: usize,
    pub generated_background_radius_y: usize,
    pub background_step_distance: f32,
    pub noise_scale: f64,
    pub tile_size: f32,
}