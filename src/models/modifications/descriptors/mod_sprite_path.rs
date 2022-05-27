use bevy::ecs::component::Component;
use bevy::prelude::{Handle, Image};

#[derive(Component)]
pub struct SpriteHandle {
    pub handle: Handle<Image>,
}