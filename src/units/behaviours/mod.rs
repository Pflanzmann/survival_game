use bevy::core::FixedTimestep;
use bevy::prelude::{App, Plugin, SystemSet};

use crate::AppState;
use crate::units::behaviours::spin_aim_behaviour_system::spin_aim_behaviour_system;
use crate::util::stage_label_helper::in_update;

mod spin_aim_behaviour_system;

pub struct BehaviourPlugin;

impl Plugin for BehaviourPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_run_criteria(FixedTimestep::step(0.2))
                        .with_system(spin_aim_behaviour_system)
                )
            );
    }
}
