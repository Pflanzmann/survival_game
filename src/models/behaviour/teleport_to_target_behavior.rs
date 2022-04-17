use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct TeleportToTargetBehavior{
    pub target : Entity,
    pub distance : f32,
    pub proximity_min : f32,
    pub proximity_max : f32,
    pub cooldown : f32,
    pub timer: f32
}
