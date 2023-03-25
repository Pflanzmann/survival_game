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
            SpriteLayer::Background => -900000000.0,
            SpriteLayer::FloorLevel => -800000000.0,
            SpriteLayer::LowGroundLevel => -700000000.0,
            SpriteLayer::GroundLevel => -600000000.0,
            SpriteLayer::AirLevel => -500000000.0,
            SpriteLayer::UILevel => -400000000.0,
        }
    }
}

impl Default for SpriteLayer {
    fn default() -> Self {
        SpriteLayer::GroundLevel
    }
}