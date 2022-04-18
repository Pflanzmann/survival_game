use bevy::prelude::{App, Plugin, SystemSet};

use crate::AppState;
use crate::input::camera_systems::setup_camera_system;
use crate::input::cmd::CmdLogicPlugin;
use crate::input::gun_mod_debug_system::gun_mod_debug_system;
use crate::input::player_control_aim_system::player_control_aim_system;
use crate::input::player_control_movement_system::player_control_movement_system;
use crate::input::toggle_pause_system::{StateTimer, toggle_pause_system};
use crate::util::stage_label_helper::in_update;

mod player_control_movement_system;
mod camera_systems;
mod gun_mod_debug_system;
mod player_control_aim_system;
mod toggle_pause_system;
mod cmd;

/// The [StateTimer] is for the [toggle_pause_system] so that it does not trigger x times per click.
/// [toggle_pause_system] is registered in every [AppState] for now.
///
/// [setup_camera_system] is called on the exit of the [AppState::MainMenu] to handle setup timings.
///
/// Every input system is handled in the update of the [AppState::InGame]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(CmdLogicPlugin)

            .init_resource::<StateTimer>()

            .add_system_set(
                in_update(
                    SystemSet::on_exit(AppState::MainMenu)
                        .with_system(setup_camera_system)
                )
            )

            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(player_control_movement_system)
                        .with_system(player_control_aim_system)
                        .with_system(gun_mod_debug_system)
                )
            )

            .add_system(toggle_pause_system);
    }
}