use bevy::prelude::{Entity, Handle, Image, Resource};

#[derive(Default, Resource)]
pub struct HudState {
    pub image_handles: Vec<Handle<Image>>,
    pub entities: Vec<Entity>,
    pub fps_timer: f32,
}