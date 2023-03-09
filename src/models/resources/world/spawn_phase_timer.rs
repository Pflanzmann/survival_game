use bevy::prelude::Resource;

#[derive(Resource)]
pub struct SpawnPhaseTimer {
    pub current_spawn_phase: usize,
    pub timer: f32,
}

impl Default for SpawnPhaseTimer {
    fn default() -> Self {
        Self { current_spawn_phase: 0, timer: 3.0 }
    }
}