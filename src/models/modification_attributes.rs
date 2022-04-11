use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct ModName {
    pub mod_name: String,
}

#[derive(Component)]
pub struct ToolTip {
    pub tooltip: String,
}

#[derive(Component)]
pub struct SpritePath {
    pub sprite_path: String,
}

#[derive(Component)]
pub struct Modification;