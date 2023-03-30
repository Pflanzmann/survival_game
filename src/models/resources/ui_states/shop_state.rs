use bevy::prelude::{Entity, Resource};
use bevy_egui::egui::TextureId;

#[derive(Default, Resource)]
pub struct ShopState {
    pub chosen_mod: Vec<ShopMod>,
}

pub struct ShopMod {
    pub entity: Entity,
    pub title: String,
    pub description: String,
    pub texture_id: TextureId,
}

impl ShopMod {
    pub fn new(entity: Entity, title: String, description: String, image_handles: TextureId) -> Self {
        ShopMod { entity, title, description, texture_id: image_handles }
    }
}
