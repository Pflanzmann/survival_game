use bevy::prelude::{App, Plugin};

use crate::world::goal::goal_activation_system::goal_activation_system;
use crate::world::goal::setup_goal_system::setup_goal_system;

mod goal_activation_system;
mod setup_goal_system;

pub struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_goal_system)
            .add_system(goal_activation_system)
        ;
    }
}
