use bevy::ecs::component::Component;
use bevy::prelude::Entity;
use bevy::utils::HashSet;

#[derive(Component, Default)]
pub struct ModRegister {
    pub register: HashSet<Entity>,
}