use bevy::core::FixedTimestep;
use bevy::prelude::{App, Plugin, SystemSet};

use crate::AppState;
use crate::units::behaviors::aim_at_closest_target_behavior_system::aim_at_closest_target_behavior_system;
use crate::units::behaviors::chase_target_behavior_system::chase_target_behavior_system;
use crate::units::behaviors::mono_directional_move_behavior_system::mono_directional_move_behavior_system;
use crate::units::behaviors::spin_aim_behavior_system::spin_aim_behavior_system;
use crate::units::behaviors::steering_behavior_system::steering_behavior_system;
use crate::units::behaviors::teleport_to_target_behavior_system::teleport_to_target_behavior_system;
use crate::units::behaviors::turn_to_target_behavior_system::turn_to_target_behavior_system;
use crate::util::stage_label_helper::{in_collision, in_update};

mod spin_aim_behavior_system;
mod teleport_to_target_behavior_system;
mod chase_target_behavior_system;
mod mono_directional_move_behavior_system;
mod aim_at_closest_target_behavior_system;
mod steering_behavior_system;
mod turn_to_target_behavior_system;

pub struct BehaviorPlugin;

impl Plugin for BehaviorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_collision(
                    SystemSet::new()
                        .with_system(steering_behavior_system)
                )
            )

            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_run_criteria(FixedTimestep::step(0.1))
                        .with_system(spin_aim_behavior_system)
                        .with_system(teleport_to_target_behavior_system)
                        .with_system(chase_target_behavior_system)
                        .with_system(mono_directional_move_behavior_system)
                        .with_system(aim_at_closest_target_behavior_system)
                        .with_system(turn_to_target_behavior_system)
                )
            );
    }
}
