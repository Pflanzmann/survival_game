use bevy::ecs::component::Component;

#[derive(Component)]
pub struct IdleAnimation {
    pub progress : f32,
    pub framecount : usize,
    pub atlas_row : usize,
    pub gameframes_until_loop : i32
}

impl IdleAnimation {

    pub fn new (progress :f32, framecount : usize, atlas_row: usize, gameframes :  i32) -> IdleAnimation{
        IdleAnimation{progress: progress,framecount: framecount, atlas_row: atlas_row, gameframes_until_loop : gameframes }
    }

}