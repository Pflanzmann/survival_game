use bevy::ecs::component::Component;

#[derive(Component)]
pub struct UnitRotation {
    pub revolutions_per_min: f32,
}