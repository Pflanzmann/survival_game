use bevy::prelude::{Plugin, SystemSet};

use crate::{App, ConsoleState};
use crate::ui::cmd::debug_window::{exit_debug_console_system, setup_debug_window};
use crate::ui::cmd::update_console_history::update_console_history;

pub mod debug_window;
mod update_console_history;

pub struct CmdUiPlugin;

impl Plugin for CmdUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(ConsoleState::Shown)
                    .with_system(setup_debug_window)
            )
            .add_system_set(
                SystemSet::on_exit(ConsoleState::Shown)
                    .with_system(exit_debug_console_system)
            )
            .add_system_set(
                SystemSet::on_update(ConsoleState::Shown)
                    .with_system(update_console_history)
            )
        ;
    }
}