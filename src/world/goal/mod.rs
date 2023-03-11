use bevy::prelude::*;

use crate::AppState;
use crate::scheduling::BaseSets;
use crate::world::goal::goal_activation_system::goal_activation_system;
use crate::world::goal::setup_goal_system::setup_goal_system;

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
            GoalSetupSystemSet
                .in_base_set(BaseSets::PostUpdate)
            // .in_schedule(OnEnter(AppState::InGame))
        );

        app.configure_set(
            GoalUpdateSystemSet
                .in_base_set(BaseSets::PostUpdate)
                .run_if(in_state(AppState::InGame))
        );

        app.add_system(setup_goal_system.in_set(GoalSetupSystemSet).in_schedule(OnEnter(AppState::InGame)));

        app.add_system(goal_activation_system.in_set(GoalUpdateSystemSet));
    }
}
