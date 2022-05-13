use bevy::ecs::component::Component;
use bevy::prelude::Vec2;

#[derive(Component)]
pub struct MonoDirectionalMoveBehavior {
    pub direction: Vec2,
}
