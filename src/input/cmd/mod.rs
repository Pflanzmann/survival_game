use bevy::prelude::{App, Plugin, SystemSet};

use crate::ConsoleState;
use crate::input::cmd::apply_mod_command::apply_mod_command;
use crate::input::cmd::cmd_input_system::cmd_input_system;
use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::util::run_criteria::on_event::on_event;

mod apply_mod_command;
mod cmd_input_system;

pub struct CmdLogicPlugin;

impl Plugin for CmdLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(on_event::<DebugCommandEvent>)

                    .with_system(apply_mod_command)
            )

            .add_system_set(
                SystemSet::on_update(ConsoleState::Shown)
                    .with_system(cmd_input_system)
            )
        ;
    }
}