use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct GrowShot {
    pub size_step: f32,
    pub damage_step: f32,
}
