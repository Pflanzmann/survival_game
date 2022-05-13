use bevy::ecs::component::Component;
use bevy::prelude::{Deref, DerefMut};

#[derive(Component, Deref, DerefMut)]
pub struct Layerable {
    pub layer: f32,
}

impl Layerable {
    pub fn new(layer: f32) -> Self {
        Layerable { layer }
    }
}
