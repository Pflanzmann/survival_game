use bevy::prelude::Resource;

#[derive(Default, Resource)]
pub struct SpawnStageState {
    pub current_spawn_phase: usize,
    pub phase_timer: f32,
    pub spawn_interval: f32,
}
