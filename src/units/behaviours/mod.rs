use bevy::core::FixedTimestep;
use bevy::prelude::{App, Plugin, SystemSet};

use crate::AppState;
use crate::units::behaviours::aim_at_closest_target_behavior_system::aim_at_closest_target_behavior_system;
use crate::units::behaviours::chase_target_behavior_system::chase_target_behavior_system;
use crate::units::behaviours::mono_directional_move_behavior_system::mono_directional_move_behavior_system;
use crate::units::rotate_unit_system::rotate_unit_system;
use crate::units::behaviours::spin_aim_behaviour_system::spin_aim_behaviour_system;
use crate::units::behaviours::teleport_to_target_behavior_system::teleport_to_target_behavior_system;
use crate::util::stage_label_helper::in_update;

mod spin_aim_behaviour_system;
mod teleport_to_target_behavior_system;
mod chase_target_behavior_system;
mod mono_directional_move_behavior_system;
mod aim_at_closest_target_behavior_system;

pub struct BehaviourPlugin;

impl Plugin for BehaviourPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_system(rotate_behavior_system)
            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_run_criteria(FixedTimestep::step(0.1))
                        .with_system(spin_aim_behaviour_system)
                        .with_system(teleport_to_target_behavior_system)
                        .with_system(chase_target_behavior_system)
                        .with_system(mono_directional_move_behavior_system)
                        .with_system(aim_at_closest_target_behavior_system)
                        .with_system(rotate_unit_system)
                )
            );
    }
}
