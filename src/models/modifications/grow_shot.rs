use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct GrowShot {
    pub damage_per_second: f32,
    pub size_per_second: f32,
}
