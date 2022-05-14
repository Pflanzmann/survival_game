use bevy::ecs::component::Component;
use bevy::prelude::{Deref, DerefMut};

#[derive(Component)]
pub struct TimeAlive {
    pub time_alive: f32,
}