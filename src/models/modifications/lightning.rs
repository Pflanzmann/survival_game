use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Clone, Component, Deserialize)]
pub struct Lightning {
    pub chance: f32,
    pub radius: f32,
    pub jump_count: i32,
    pub lightning_sprite_path: String,
    pub lightning_width: f32,
    pub sprite_time_alive: f32,
}
