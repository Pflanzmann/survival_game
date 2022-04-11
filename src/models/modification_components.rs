use bevy::ecs::component::Component;
use bevy::prelude::Entity;
use serde::{Deserialize};

#[derive(Component)]
pub struct ModContainerSlot {
    pub container_entity: Entity,
}

#[derive(Component)]
pub struct ModContainer;

#[derive(Copy, Clone, Hash, Component, Deserialize)]
pub struct CurveShot;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct GrowShot {
    pub size_step: f32,
    pub damage_step: f32,
}

#[derive(Copy, Clone, Component, Deserialize)]
pub struct SplitShot;

