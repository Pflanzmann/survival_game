use bevy::ecs::component::Component;
use bevy::prelude::Deref;
use serde::Deserialize;

#[derive(Component, Deref, Deserialize)]
pub struct Price(pub i32);