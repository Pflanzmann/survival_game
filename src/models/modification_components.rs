use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct ModContainerSlot {
    pub container_entity: Entity,
}

#[derive(Component)]
pub struct ModContainer;

