use bevy::ecs::component::Component;
use bevy::math::Vec3;

#[derive(Component)]
pub struct MoveDirection {
    pub direction: Vec3,
}
