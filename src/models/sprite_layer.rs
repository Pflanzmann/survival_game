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
        return match *self {
            SpriteLayer::BackGround => 0.0,
            SpriteLayer::FloorLevel => 1.0,
            SpriteLayer::LowGroundLevel => 2.0,
            SpriteLayer::GroundLevel => 3.0,
            SpriteLayer::AirLevel => 4.0,
            SpriteLayer::UILevel => 5.0,
        };
    }
}