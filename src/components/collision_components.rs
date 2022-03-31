use bevy::ecs::component::Component;
use bevy::prelude::Vec3;

#[derive(Component)]
pub struct Collider;


#[derive(Component)]
pub struct Collisions {
    pub collisions: Vec<(Vec3)>,
}
