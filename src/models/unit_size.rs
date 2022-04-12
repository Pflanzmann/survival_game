use bevy::ecs::component::Component;
use bevy::prelude::Vec2;

#[derive(Component)]
pub struct UnitSize {
    pub collider_size: Vec2,
}
