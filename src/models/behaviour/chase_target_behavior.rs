use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct ChaseTargetBehavior{
    pub target : Entity,
    pub proximity : f32,
}
