use bevy::ecs::component::Component;

use crate::Vec2;

#[derive(Component)]
pub struct EnemyAi;

#[derive(Component)]
pub struct Speed {
    pub speed: f32,
}

#[derive(Component)]
pub struct Size {
    pub size: Vec2,
}
