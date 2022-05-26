use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct TurnToTargetBehavior {
    pub target: Entity,
    pub revolutions_per_min: f32,
}
