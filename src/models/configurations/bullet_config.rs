use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
pub struct BulletConfig {
    pub sprite_custom_size_x: f32,
    pub sprite_custom_size_y: f32,
    pub sprite_path: String,
    pub speed: f32,
    pub damage: f32,
    pub range: f32,
    pub hit_limit: f32,
}