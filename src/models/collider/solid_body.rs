use bevy::ecs::component::Component;

#[derive(Component)]
pub struct SolidBody {
    pub weight: f32,
}