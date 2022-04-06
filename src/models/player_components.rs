use bevy::ecs::component::Component;

use crate::Vec3;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct AimDirection {
    pub direction: Vec3,
}

