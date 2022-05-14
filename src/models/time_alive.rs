use bevy::ecs::component::Component;

#[derive(Component)]
pub struct TimeAlive {
    pub time_alive: f32,
}