use bevy::ecs::component::Component;
use bevy::prelude::Entity;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct Slime;

#[derive(Component)]
pub struct SlimeUnit;

#[derive(Component)]
pub struct SlimeOwner {
    pub owner : Entity,
}