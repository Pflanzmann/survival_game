use bevy::ecs::component::Component;

#[derive(Component)]
pub struct DamagedEffect {
    pub timer: f32,
}

impl DamagedEffect {
    pub fn new() -> Self {
        Self { timer: 0.0 }
    }
}
