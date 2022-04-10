use bevy::prelude::{*, App, CoreStage, IntoSystem, Plugin, SystemSet};

use crate::AppState;
use crate::navigation::execute_state_switch_system::execute_state_switch_system;
use crate::navigation::stage_label_system::{collision_label_system, first_label_system, post_update_label_system, pre_update_label_system, update_label_system};
use crate::navigation::trigger_enter_main_system::trigger_enter_main_system;
use crate::util::stage_label_helper::{in_first, in_update};

pub mod execute_state_switch_system;
pub mod trigger_enter_main_system;
pub mod stage_label_system;

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(first_label_system.label("first").before("collision"))
            .add_system(collision_label_system.label("collision").after("first").before("preUpdate"))
            .add_system(pre_update_label_system.label("preUpdate").after("collision").before("Update"))
            .add_system(update_label_system.label("Update").after("preUpdate").before("PostUpdate"))
            .add_system(post_update_label_system.label("PostUpdate").after("Update"))

            .add_system_set(
                in_first(
                    SystemSet::new()
                        .with_system(execute_state_switch_system)
                )
            )

            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::Pre)
                        .with_system(trigger_enter_main_system)
                )
            );
    }
}