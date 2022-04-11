use bevy::ecs::component::Component;
use bevy::prelude::Entity;
use crate::{Handle, Image};

#[derive(Component)]
pub struct ModName {
    pub mod_name: String,
}

#[derive(Component)]
pub struct ToolTip {
    pub tooltip: String,
}

#[derive(Component)]
pub struct ModSpriteHandler {
    pub sprite: Handle<Image>,
}

