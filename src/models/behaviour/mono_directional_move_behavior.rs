use bevy::ecs::component::Component;
use bevy::prelude::{Vec3};

#[derive(Component)]
pub struct MonoDirectionalMoveBehavior {
    pub direction: Vec3,
}
