use bevy::ecs::component::Component;
use bevy::prelude::Vec2;

#[derive(Component)]
pub struct TeleportingScript {
    pub progress: f32,
    pub duration: f32,
    pub target_pos: Vec2,
    pub did_port: bool,
}

impl TeleportingScript {
    pub fn new(target: Vec2, duration: f32) -> Self {
        Self { progress: 0.0, duration: duration, target_pos: target, did_port: false }
    }
}