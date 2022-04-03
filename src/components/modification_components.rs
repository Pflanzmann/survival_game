use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct ModContainerSlot {
    pub container_entity: Entity,
}

#[derive(Component)]
pub struct ModContainer;

#[derive(Copy, Clone, Hash, Component)]
pub struct CurveShot;

#[derive(Copy, Clone, Component)]
pub struct GrowShot {
    pub grow_step: f32,
}

#[derive(Copy, Clone, Component)]
pub struct SplitShot;