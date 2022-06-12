use std::collections::HashMap;
use bevy::prelude::Resource;

#[derive(Default, Resource)]
pub struct InfoWindowState {
    pub infos: HashMap<String, String>,
    pub fps_counter: String,
    pub fps_timer: f32,
}