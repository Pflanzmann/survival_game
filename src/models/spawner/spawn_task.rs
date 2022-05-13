use bevy::math::Vec3;

pub struct SpawnTask {
    position: Vec3,
}

impl SpawnTask {
    pub fn new(position: Vec3) -> Self {
        SpawnTask { position }
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }
}