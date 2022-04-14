use bevy::ecs::component::Component;

#[derive(Component)]
pub struct GoldValue {
    pub gold_value: i32,
}
