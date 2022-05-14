use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Clone, Component, Deserialize)]
pub struct ExplosionShot {
    pub explosion_sprite_path: String,
    pub radius: f32,
    pub explosion_time_alive: f32,
}