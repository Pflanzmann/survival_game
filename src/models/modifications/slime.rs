use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct Slime;

#[derive(Component)]
pub struct SlimeUnit;