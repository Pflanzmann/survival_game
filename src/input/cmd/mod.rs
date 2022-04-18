use bevy::prelude::{App, Plugin, SystemSet};

use crate::input::cmd::apply_mod_command::apply_mod_command;
use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::util::run_criteria::on_event::on_event;

mod apply_mod_command;

pub struct CmdPlugin;

impl Plugin for CmdPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(on_event::<DebugCommandEvent>)

                    .with_system(apply_mod_command)
            )
        ;
    }
}