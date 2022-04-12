use bevy::ecs::component::Component;
use bevy::prelude::Vec3;

#[derive(Component)]
pub struct AimDirection {
    pub direction: Vec3,
}

