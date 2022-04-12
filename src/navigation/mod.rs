use bevy::prelude::{*, App, Plugin, SystemSet};

use crate::AppState;
use crate::navigation::execute_state_switch_system::execute_state_switch_system;
use crate::navigation::stage_label_system::{collision_label_system, first_label_system, post_update_label_system, pre_update_label_system, update_label_system};
use crate::navigation::trigger_enter_main_system::trigger_enter_main_system;
use crate::util::stage_label_helper::{in_first, in_update};

pub mod execute_state_switch_system;
pub mod trigger_enter_main_system;
pub mod stage_label_system;

pub struct NavigationPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
pub enum ScheduleLabel {
    First,
    Collision,
    PreUpdate,
    Update,
    PostUpdate
}

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(first_label_system.label(ScheduleLabel::First).before(ScheduleLabel::Collision))
            .add_system(collision_label_system.label(ScheduleLabel::Collision).after(ScheduleLabel::First).before(ScheduleLabel::PreUpdate))
            .add_system(pre_update_label_system.label(ScheduleLabel::PreUpdate).after(ScheduleLabel::Collision).before(ScheduleLabel::Update))
            .add_system(update_label_system.label(ScheduleLabel::Update).after(ScheduleLabel::PreUpdate).before(ScheduleLabel::PostUpdate))
            .add_system(post_update_label_system.label(ScheduleLabel::PostUpdate).after(ScheduleLabel::Update))

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