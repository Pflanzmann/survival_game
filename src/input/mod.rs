use bevy::prelude::{App, in_state, IntoSystemSetConfig, Plugin, SystemSet};
use bevy::prelude::IntoSystemConfig;

use crate::AppState;
use crate::input::camera_systems::setup_camera_system;
use crate::input::cmd::CmdCommandsPlugin;
use crate::input::player_control_aim_system::player_control_aim_system;
use crate::input::player_control_movement_system::player_control_movement_system;
use crate::input::toggle_pause_system::{StateTimer, toggle_pause_system};
use crate::scheduling::BaseSets;

mod player_control_movement_system;
mod camera_systems;
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

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct InputSystemSet;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            InputSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(in_state(AppState::InGame))
        );

        app.add_plugin(CmdCommandsPlugin);

        app.init_resource::<StateTimer>();

        app.add_startup_system(setup_camera_system);

        app
            .add_system(player_control_movement_system.in_set(InputSystemSet))
            .add_system(player_control_aim_system.in_set(InputSystemSet))
            .add_system(toggle_pause_system.in_set(InputSystemSet));
    }
}