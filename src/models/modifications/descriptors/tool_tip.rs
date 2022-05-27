use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Component, Deserialize, Clone)]
pub struct ToolTip {
    pub tooltip: String,
}
