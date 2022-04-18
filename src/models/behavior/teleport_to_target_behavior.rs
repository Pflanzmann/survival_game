use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct TeleportToTargetBehavior {
    pub target: Entity,
    pub distance: f32,
    pub proximity_min: f32,
    pub proximity_max: f32,
    pub cooldown: f32,
    pub timer: f32,
}

impl TeleportToTargetBehavior {
    pub fn new(target: Entity, distance: f32, proximity_min: f32, proximity_max: f32, cooldown: f32) -> Self {
        TeleportToTargetBehavior { target, distance, proximity_min, proximity_max, cooldown, timer: 0.0 }
    }
}
