use bevy::ecs::component::Component;

#[derive(Component)]
pub struct MoveAnimation {
    pub progress : f32,
    pub framecount : usize,
    pub atlas_row : usize,
    pub gameframes_until_loop : i32
}
