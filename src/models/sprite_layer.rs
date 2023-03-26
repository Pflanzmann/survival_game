use serde::Deserialize;

#[derive(Deserialize, Copy, Clone)]
pub enum SpriteLayer {
    Background,
    FloorLevel,
    LowGroundLevel,
    GroundLevel,
    AirLevel,
    UILevel,
}

impl SpriteLayer {
    pub fn get_layer_z(&self) -> f32 {
        match *self {
            SpriteLayer::Background => -30000.0,
            SpriteLayer::FloorLevel => -20000.0,
            SpriteLayer::LowGroundLevel => -10000.0,
            SpriteLayer::GroundLevel => 0.0,
            SpriteLayer::AirLevel => 10000.0,
            SpriteLayer::UILevel => 20000.0,
        }
    }
}

impl Default for SpriteLayer {
    fn default() -> Self {
        SpriteLayer::GroundLevel
    }
}