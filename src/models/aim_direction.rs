use bevy::ecs::component::Component;
use bevy::prelude::Vec2;

#[derive(Component, Default)]
pub struct AimDirection {
    pub direction: Vec2,
}

