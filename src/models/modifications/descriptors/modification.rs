use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Component, Deserialize)]
pub struct Modification;