use bevy::prelude::{App, in_state, IntoSystemConfig, on_event, Plugin, SystemSet};
use bevy::prelude::IntoSystemSetConfig;

use crate::ConsoleState;
use crate::input::cmd::add_gold_command::add_gold_command;
use crate::input::cmd::apply_mod_command::apply_mod_command;
use crate::input::cmd::god_mode_command::god_mode_command;
use crate::input::cmd::open_shop_command::open_shop_command;
use crate::input::cmd::remove_mod_command::remove_mod_command;
use crate::input::cmd::toggle_spawner_command::toggle_spawner_command;
use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::scheduling::BaseSets;

mod apply_mod_command;
mod god_mode_command;
mod remove_mod_command;
mod open_shop_command;
mod add_gold_command;
mod toggle_spawner_command;

pub struct CmdCommandsPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CmdCommandsSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CmdInputSystemSet;

impl Plugin for CmdCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            CmdCommandsSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(on_event::<DebugCommandEvent>())
        );

        app.configure_set(
            CmdInputSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(in_state(ConsoleState::Shown))
        );

        app
            .add_system(apply_mod_command.in_set(CmdCommandsSystemSet))
            .add_system(remove_mod_command.in_set(CmdCommandsSystemSet))
            .add_system(god_mode_command.in_set(CmdCommandsSystemSet))
            .add_system(open_shop_command.in_set(CmdCommandsSystemSet))
            .add_system(add_gold_command.in_set(CmdCommandsSystemSet))
            .add_system(toggle_spawner_command.in_set(CmdCommandsSystemSet));

        // app.add_system(cmd_input_system.in_set(CmdInputSystemSet));
    }
}
