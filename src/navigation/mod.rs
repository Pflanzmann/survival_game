use bevy::prelude::{App, CoreStage, Plugin, SystemSet};
use crate::AppState;

use crate::navigation::execute_state_switch_system::execute_state_switch_system;
use crate::navigation::trigger_enter_main_system::trigger_enter_main_system;

pub mod execute_state_switch_system;
pub mod trigger_enter_main_system;

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_to_stage(CoreStage::First, execute_state_switch_system)

            .add_system_set(SystemSet::on_update(AppState::Pre)
                                .with_system(trigger_enter_main_system)
        );
    }
}