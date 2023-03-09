use bevy::prelude::Resource;

#[derive(Resource)]
pub struct DebugCommandEvent {
    pub debug_command: String,

    pub key: String,
    pub values: Vec<String>,
    pub arguments: Vec<String>,
}
