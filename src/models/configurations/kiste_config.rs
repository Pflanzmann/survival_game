use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
pub struct KisteConfig {
    pub sprite_custom_size_x: f32,
    pub sprite_custom_size_y: f32,
    pub sprite_path: String,
}