use bevy::ecs::component::Component;

#[derive(Component, Default)]
pub struct GoldStorage {
    pub number: i32,
}
