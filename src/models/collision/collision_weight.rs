use bevy::ecs::component::Component;

#[derive(Component)]
pub struct CollisionWeight {
    pub weight: f32,
}