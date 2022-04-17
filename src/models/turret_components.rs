use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct TurretUnit;

#[derive(Component)]
pub struct TurretOwner {
    pub owner : Entity,
}