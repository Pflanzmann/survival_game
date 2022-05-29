use bevy::ecs::component::Component;

#[derive(Component)]
pub struct FadeAnimation {
    pub fade_time: f32,
}