use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Copy, Clone, Component)]
pub struct Bullet {
    pub source_entity: Entity,
}