use bevy::ecs::component::Component;
use bevy::prelude::Vec2;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct DeathBall {
    pub unit_size: Vec2,
    pub reload: f32,

    pub rotation_distance: f32,
    pub revolutions_per_min: f32,
}

#[derive(Component)]
pub struct DeathBallUnit;