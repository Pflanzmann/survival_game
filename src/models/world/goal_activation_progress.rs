use bevy::ecs::component::Component;

#[derive(Component)]
pub struct GoalActivationProgress {
    pub progress: f32,
}