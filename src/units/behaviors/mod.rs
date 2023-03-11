use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_fixed_timer;

use crate::AppState;
use crate::scheduling::BaseSets;
use crate::units::behaviors::aim_at_closest_target_behavior_system::aim_at_closest_target_behavior_system;
use crate::units::behaviors::chase_target_behavior_system::chase_target_behavior_system;
use crate::units::behaviors::mono_directional_move_behavior_system::mono_directional_move_behavior_system;
use crate::units::behaviors::spin_aim_behavior_system::spin_aim_behavior_system;
use crate::units::behaviors::steering_behavior_system::steering_behavior_system;
use crate::units::behaviors::teleport_to_target_behavior_system::teleport_to_target_behavior_system;
use crate::units::behaviors::turn_to_target_behavior_system::turn_to_target_behavior_system;

mod spin_aim_behavior_system;
mod teleport_to_target_behavior_system;
mod chase_target_behavior_system;
mod mono_directional_move_behavior_system;
mod aim_at_closest_target_behavior_system;
mod steering_behavior_system;
mod turn_to_target_behavior_system;

pub struct BehaviorPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct BehaviorFixedSystemSet;

impl Plugin for BehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            BehaviorFixedSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(in_state(AppState::InGame))
                .run_if(on_fixed_timer(Duration::from_millis(10)))
        );

        app
            .add_system(steering_behavior_system.in_set(BehaviorFixedSystemSet))
            .add_system(spin_aim_behavior_system.in_set(BehaviorFixedSystemSet))
            .add_system(teleport_to_target_behavior_system.in_set(BehaviorFixedSystemSet))
            .add_system(chase_target_behavior_system.in_set(BehaviorFixedSystemSet))
            .add_system(mono_directional_move_behavior_system.in_set(BehaviorFixedSystemSet))
            .add_system(aim_at_closest_target_behavior_system.in_set(BehaviorFixedSystemSet))
            .add_system(turn_to_target_behavior_system.in_set(BehaviorFixedSystemSet));
    }
}
