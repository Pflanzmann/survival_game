pub enum SpriteLayer {
    BackGround,
    FloorLevel,
    LowGroundLevel,
    GroundLevel,
    AirLevel,
    UILevel,
}

impl SpriteLayer {
    pub fn get_layer_z(&self) -> f32 {
        match *self {
            SpriteLayer::BackGround => -9000000000000.0,
            SpriteLayer::FloorLevel => -8000000000000.0,
            SpriteLayer::LowGroundLevel => -7000000000000.0,
            SpriteLayer::GroundLevel => -6000000000000.0,
            SpriteLayer::AirLevel => -5000000000000.0,
            SpriteLayer::UILevel => -4000000000000.0,
        }
    }
}