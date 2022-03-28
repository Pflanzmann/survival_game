use bevy::ecs::component::Component;
use bevy::math::Vec3;

use crate::Vec2;

#[derive(Component)]
pub struct EnemyAi;

#[derive(Component)]
pub struct Speed {
    pub speed: f32,
}

#[derive(Component)]
pub struct Direction{
    pub direction: Vec3
}

#[derive(Component)]
pub struct Size {
    pub size: Vec2,
}
