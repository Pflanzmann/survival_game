use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct BurningShot;

#[derive(Component)]
pub struct BurningShotUnit;
