use bevy::ecs::component::Component;
use bevy::prelude::Vec2;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct Radiation {
    pub unit_size: Vec2,
    pub damage: f32,
    pub damage_interval: f32,
}

#[derive(Component)]
pub struct RadiationUnit;