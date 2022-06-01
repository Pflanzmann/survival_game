use bevy::prelude::{App, Plugin, SystemSet};

use crate::AppState;
use crate::world::goal::goal_activation_system::goal_activation_system;
use crate::world::goal::setup_goal_system::setup_goal_system;

mod goal_activation_system;
mod setup_goal_system;

pub struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(setup_goal_system)
                    .with_system(goal_activation_system)
            )
        ;
    }
}
