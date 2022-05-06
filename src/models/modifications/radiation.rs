use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct Radiation;

#[derive(Component)]
pub struct RadiationUnit;