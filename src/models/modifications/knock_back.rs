use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct KnockBack {
    pub push_duration: f32,
    pub push_force: f32,
}