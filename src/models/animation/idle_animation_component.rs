use bevy::ecs::component::Component;

#[derive(Component)]
pub struct IdleAnimation {
    pub progress: f32,
    pub animation_frame_count: usize,
    pub atlas_row: usize,
    pub duration: f32,
}

impl IdleAnimation {
    pub fn new(progress: f32, animation_frame_count: usize, atlas_row: usize, duration: f32) -> IdleAnimation {
        IdleAnimation { progress, animation_frame_count, atlas_row, duration }
    }
}