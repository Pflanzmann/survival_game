use bevy::ecs::component::Component;
use bevy::math::Vec3;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct TeleportingScript {
    pub timer: f32,
    pub target_pos: Vec3,
    pub did_port: bool
}

impl TeleportingScript {
    pub fn new(target: Vec3) -> Self {
        Self { timer: 0.0, target_pos: target, did_port: false }
    }
}