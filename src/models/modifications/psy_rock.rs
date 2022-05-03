use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct PsyRock;

#[derive(Component)]
pub struct PsyRockUnit;