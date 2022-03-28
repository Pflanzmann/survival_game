use bevy::prelude::Plugin;
use crate::ai::enemy_movement_system::enemy_movement_system;
use crate::ai::spawn_system::spawn_system;
use crate::App;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_system)
            .add_system(enemy_movement_system);
    }
}