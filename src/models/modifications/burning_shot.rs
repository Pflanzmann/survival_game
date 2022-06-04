use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct BurningShot {
    pub debuff_duration: f32,
    pub damage_interval: f32,
    pub damage_per_tick: f32,
}
