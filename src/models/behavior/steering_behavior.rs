use bevy::prelude::*;

#[derive(Component, Default)]
pub struct SteeringBehavior {
    pub direction: Vec2,
}
