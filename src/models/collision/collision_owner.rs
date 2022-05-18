use bevy::ecs::component::Component;
use bevy::prelude::Deref;
use bevy::prelude::Entity;

#[derive(Component, Deref)]
pub struct ColliderWeight(pub Entity);
