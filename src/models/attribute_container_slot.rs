use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct AttributeContainerSlot {
    pub container_entity: Entity,
}

