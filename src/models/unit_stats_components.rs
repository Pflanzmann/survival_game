use bevy::ecs::component::Component;
use bevy::math::Vec3;

use crate::Vec2;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct MoveDirection {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct UnitSize {
    pub collider_size: Vec2,
}
