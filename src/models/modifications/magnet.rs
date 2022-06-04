use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct Magnet {
    pub pulse_time: f32,
    pub pulse_timer: f32,
    pub radius: f32,
    pub push_duration: f32,
    pub push_force: f32,
}