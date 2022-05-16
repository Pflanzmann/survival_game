use bevy::ecs::component::Component;
use bevy::prelude::Vec2;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct Shield {
    pub unit_size: Vec2,
    pub damage: f32,
    pub damage_interval: f32,
    pub push_duration: f32,
    pub push_force: f32,
    pub rotation_distance: f32,
    pub revolutions_per_min: f32,
}

#[derive(Component)]
pub struct ShieldUnit;
