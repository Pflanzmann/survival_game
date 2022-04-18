use bevy::ecs::component::Component;
use bevy::prelude::Entity;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct Owner {
    pub entity: Entity,
}

impl Owner {
    pub fn new(entity: Entity) -> Self {
        Owner { entity }
    }
}
