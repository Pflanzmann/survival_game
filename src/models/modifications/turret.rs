use bevy::ecs::component::Component;
use bevy::prelude::Vec2;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct Turret {
    pub unit_size: Vec2,
    pub reload: f32,
    pub teleport_distance: f32,
    pub teleport_proximity_min: f32,
    pub teleport_proximity_max: f32,
    pub teleport_cooldown: f32,
}

#[derive(Component)]
pub struct TurretUnit;
