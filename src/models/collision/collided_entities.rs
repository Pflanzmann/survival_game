use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component, Default, Clone)]
pub struct CollidedEntities {
    pub collisions: Vec<Entity>,
}