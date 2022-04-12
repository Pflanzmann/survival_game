use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Heal {
    pub amount: f32,
}
