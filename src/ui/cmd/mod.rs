use bevy::prelude::{Plugin, SystemSet};

use crate::{App, ConsoleState};
use crate::ui::cmd::debug_window::{exit_debug_console_system, setup_debug_info_window, setup_debug_window};
use crate::ui::cmd::spawn_collision_boxes::{despawn_collision_boxes, init_collision_boxes, spawn_collision_boxes};
use crate::ui::cmd::update_console_history::update_console_history;
use crate::util::stage_label_helper::{in_collision, in_update};

mod debug_window;
mod update_console_history;
pub mod spawn_collision_boxes;

pub struct CmdUiPlugin;

impl Plugin for CmdUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_exit(ConsoleState::Hidden)
                    .with_system(setup_debug_window)
                    .with_system(setup_debug_info_window)
            )

            .add_system_set(
                SystemSet::on_exit(ConsoleState::Shown)
                    .with_system(exit_debug_console_system)
            )
            .add_system_set(
                SystemSet::on_update(ConsoleState::Shown)
                    .with_system(update_console_history)
            )

            .add_system_set(
                in_update(
                    SystemSet::on_enter(ConsoleState::Shown)
                        .with_system(init_collision_boxes)
                )
            )

            .add_system_set(
                in_update(
                    SystemSet::on_update(ConsoleState::Shown)
                        .with_system(spawn_collision_boxes)
                )
            )

            .add_system_set(
                in_collision(
                    SystemSet::on_exit(ConsoleState::Shown)
                        .with_system(despawn_collision_boxes)
                )
            )
        ;
    }
}
