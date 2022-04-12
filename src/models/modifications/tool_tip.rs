use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Component, Deserialize)]
pub struct ToolTip {
    pub tooltip: String,
}

#[derive(Deserialize)]
pub struct ModSpriteHandlerHelper {
    pub sprite: String,
}