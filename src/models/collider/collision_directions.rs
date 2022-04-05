use bevy::ecs::component::Component;
use bevy::prelude::Vec3;

#[derive(Component)]
pub struct CollisionDirections {
    pub collisions: Vec<Vec3>,
}