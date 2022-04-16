use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct TeleportToTargetBehavior;

#[derive(Component)]
pub struct TeleportTargetPointer{
    pub target : Entity,
}