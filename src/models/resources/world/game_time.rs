use bevy::prelude::Resource;

#[derive(Default, Resource)]
pub struct GameTime {
    pub time_in_seconds: f64,
}