use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Component, Deserialize)]
pub struct ToolTip {
    pub tooltip: String,
}
