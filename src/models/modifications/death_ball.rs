use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct DeathBall {
    pub rotation_distance: f32,
    pub rotation_speed: f32,
}