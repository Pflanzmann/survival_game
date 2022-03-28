use bevy::prelude::{App, Plugin};
use bevy::render::camera::camera_system;
use crate::input::enemy_player_collision_system::player_control_system;
use crate::input::setup_player_input::setup_player_input;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_player_input)
            .add_system(player_control_system);
    }
}