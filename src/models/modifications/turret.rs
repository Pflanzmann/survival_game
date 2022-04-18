use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct Turret;

#[derive(Component)]
pub struct TurretUnit;
