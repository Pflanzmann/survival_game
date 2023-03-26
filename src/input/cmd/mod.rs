use bevy::prelude::{App, in_state, IntoSystemConfig, on_event, Plugin, SystemSet};
use bevy::prelude::IntoSystemSetConfig;

use crate::ConsoleState;
use crate::input::cmd::add_gold_command::{add_gold_command, push_gold_info};
use crate::input::cmd::apply_mod_command::{apply_mod_command, push_apply_mod_info};
use crate::input::cmd::god_mode_command::{god_mode_command, push_god_mode_info};
use crate::input::cmd::open_shop_command::{open_shop_command, push_open_shop_info};
use crate::input::cmd::remove_mod_command::{push_remove_mod_info, remove_mod_command};
use crate::input::cmd::toggle_spawner_command::{push_toggle_spawner_info, toggle_spawner_command};
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

        app
            .add_system(push_gold_info.in_set(CmdCommandsSystemSet))
            .add_system(push_apply_mod_info.in_set(CmdCommandsSystemSet))
            .add_system(push_god_mode_info.in_set(CmdCommandsSystemSet))
            .add_system(push_open_shop_info.in_set(CmdCommandsSystemSet))
            .add_system(push_remove_mod_info.in_set(CmdCommandsSystemSet))
            .add_system(push_toggle_spawner_info.in_set(CmdCommandsSystemSet));

        // app.add_system(cmd_input_system.in_set(CmdInputSystemSet));
    }
}
