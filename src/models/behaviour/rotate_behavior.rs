use bevy::ecs::component::Component;

#[derive(Component)]
pub struct UnitRotation {
    pub angle: f32,
}