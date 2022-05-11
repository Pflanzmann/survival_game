use bevy::ecs::component::Component;
use bevy::prelude::{Deref, DerefMut, Entity};

#[derive(Component, Deref, DerefMut)]
pub struct ColliderOwner(pub Entity);
