use bevy::prelude::*;

use crate::AppState;
use crate::scheduling::BaseSets;

mod goal_activation_system;
mod setup_goal_system;

pub struct GoalPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct GoalSetupSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct GoalUpdateSystemSet;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            GoalUpdateSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(in_state(AppState::InGame))
        );

        // app.add_system(setup_goal_system.in_schedule(OnEnter(AppState::MainMenu)));
        //
        // app.add_system(goal_activation_system.in_set(GoalUpdateSystemSet));
    }
}
