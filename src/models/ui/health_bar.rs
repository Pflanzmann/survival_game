use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct HealthBar {
    pub owner: Entity,
}
