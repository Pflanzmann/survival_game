use bevy::ecs::component::Component;
use bevy::prelude::{Handle, Image};
use serde::Deserialize;

#[derive(Component)]
pub struct ModSpriteHandler {
    pub sprite: Handle<Image>,
}

#[derive(Deserialize)]
pub struct ModSpriteHandlerHelper {
    pub sprite: String,
}