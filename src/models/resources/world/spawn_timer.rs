use bevy::prelude::Resource;

#[derive(Default, Resource)]
pub struct SpawnIntervalTimer {
    pub timer: f32,
}
