use bevy::prelude::{App, Plugin};

use crate::world::level::goal_activation_system::goal_activation_system;

mod goal_activation_system;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(goal_activation_system)
        ;
    }
}
