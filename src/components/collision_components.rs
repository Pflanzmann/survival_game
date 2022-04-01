use bevy::ecs::component::Component;
use bevy::prelude::Vec3;
use crate::Entity;

#[derive(Component)]
pub struct Collider;


#[derive(Component)]
pub struct CollisionDirections {
    pub collisions: Vec<Vec3>,
}

#[derive(Component)]
pub struct CollidedEntities {
    pub collisions: Vec<Entity>,
}