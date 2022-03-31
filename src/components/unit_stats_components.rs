use bevy::ecs::component::Component;
use bevy::math::Vec3;

use crate::Vec2;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct MoveSpeed {
    pub move_speed: f32,
}

#[derive(Component)]
pub struct Health {
    pub current_health: f32,
    pub max_health: f32,
}

#[derive(Component)]
pub struct Damage {
    pub damage: f32,
}

#[derive(Component)]
pub struct FacingDirection {
    pub facing_direction: Vec3,
}

#[derive(Component)]
pub struct UnitSize {
    pub collider_size: Vec2,
}
