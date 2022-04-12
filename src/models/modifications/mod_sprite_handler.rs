use bevy::ecs::component::Component;
use serde::Deserialize;

use crate::{Handle, Image};

#[derive(Component)]
pub struct ModSpriteHandler {
    pub sprite: Handle<Image>,
}