use bevy::ecs::component::Component;

#[derive(Component)]
pub struct MoveAnimationSide {
    pub progress: f32,
    pub framecount: usize,
    pub atlas_row: usize,
    pub gameframes_until_loop: i32,
}

#[derive(Component)]
pub struct MoveAnimationUp {
    pub progress: f32,
    pub framecount: usize,
    pub atlas_row: usize,
    pub gameframes_until_loop: i32,
}

#[derive(Component)]
pub struct MoveAnimationDown {
    pub progress: f32,
    pub framecount: usize,
    pub atlas_row: usize,
    pub gameframes_until_loop: i32,
}

impl MoveAnimationSide {
    pub fn new(progress: f32, framecount: usize, atlas_row: usize, gameframes: i32) -> Self {
        Self { progress: progress, framecount: framecount, atlas_row: atlas_row, gameframes_until_loop: gameframes }
    }
}

impl MoveAnimationUp {
    pub fn new(progress: f32, framecount: usize, atlas_row: usize, gameframes: i32) -> Self {
        Self { progress: progress, framecount: framecount, atlas_row: atlas_row, gameframes_until_loop: gameframes }
    }
}

impl MoveAnimationDown {
    pub fn new(progress: f32, framecount: usize, atlas_row: usize, gameframes: i32) -> Self {
        Self { progress: progress, framecount: framecount, atlas_row: atlas_row, gameframes_until_loop: gameframes }
    }
}