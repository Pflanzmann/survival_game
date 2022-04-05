use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component, Default)]
pub struct CollidedEntities {
    pub collisions: Vec<Entity>,
}