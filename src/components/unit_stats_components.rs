use bevy::ecs::component::Component;
use bevy::math::Vec3;

use crate::Vec2;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Speed {
    pub speed: f32,
}

#[derive(Component)]
pub struct Health {
    pub health: f32,
}

#[derive(Component)]
pub struct Damage {
    pub damage: f32,
}

#[derive(Component)]
pub struct Direction {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct ColliderSize {
    pub collider_size: Vec2,
}
