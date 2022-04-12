use bevy::ecs::component::Component;
use crate::Vec2;

#[derive(Component)]
pub struct UnitSize {
    pub collider_size: Vec2,
}
