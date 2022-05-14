use bevy::ecs::component::Component;
use bevy::prelude::Vec2;

#[derive(Component)]
pub struct UnitPush {
    pub direction: Vec2,
    pub duration: f32,
    pub force: f32,
}
