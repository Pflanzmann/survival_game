use bevy::ecs::component::Component;

#[derive(Component)]
pub struct ColliderWeight {
    pub weight: f32,
}