use bevy::prelude::{App, Plugin, SystemSet};

use crate::ConsoleState;
use crate::input::cmd::add_gold_command::add_gold_command;
use crate::input::cmd::apply_mod_command::apply_mod_command;
use crate::input::cmd::cmd_input_system::cmd_input_system;
use crate::input::cmd::god_mode_command::god_mode_command;
use crate::input::cmd::open_shop_command::open_shop_command;
use crate::input::cmd::remove_mod_command::remove_mod_command;
use crate::input::cmd::toggle_spawner_command::toggle_spawner_command;
use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::util::run_criteria::on_event::on_event;

mod apply_mod_command;
mod cmd_input_system;
mod god_mode_command;
mod remove_mod_command;
mod open_shop_command;
mod add_gold_command;
mod toggle_spawner_command;

pub struct CmdLogicPlugin;

impl Plugin for CmdLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(on_event::<DebugCommandEvent>)

                    .with_system(apply_mod_command)
                    .with_system(remove_mod_command)
                    .with_system(god_mode_command)
                    .with_system(open_shop_command)
                    .with_system(add_gold_command)
                    .with_system(toggle_spawner_command)
            )

            .add_system_set(
                SystemSet::on_update(ConsoleState::Shown)
                    .with_system(cmd_input_system)
            )
        ;
    }
}
