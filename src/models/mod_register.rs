use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component, Default)]
pub struct ModRegister {
    pub register: Vec<Entity>,
}