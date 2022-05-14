use serde::Deserialize;

#[derive(Deserialize)]
pub struct SpriteBundleWrapper {
    pub path: String,
    pub sprite_x: f32,
    pub sprite_y: f32,
}
