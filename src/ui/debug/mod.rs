use std::time::Duration;

use bevy::app::IntoSystemAppConfig;
use bevy::prelude::*;
use bevy::time::common_conditions::on_fixed_timer;

use crate::{App, ConsoleState};
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::health::Health;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::reload::Reload;
use crate::models::unit_attributes::unit_size::UnitSize;
use crate::scheduling::BaseSets;
use crate::ui::debug::show_debug_window_system::show_debug_window_system;
use crate::ui::debug::show_info_attribute_value_system::update_info_attribute_value_system;
use crate::ui::debug::show_info_gun_attribute_value_system::update_info_gun_attribute_value_system;
use crate::ui::debug::update_info_fps_system::show_info_fps_system;
use crate::ui::debug::show_info_info_system::show_info_window_system;
use crate::ui::debug::spawn_collision_boxes::{CollisionBox, init_collision_boxes, init_new_collision_boxes};
use crate::ui::debug::update_console_history::update_console_history;
use crate::util::helper_systems::despawn_recursive_system::despawn_recursive_system;

mod show_debug_window_system;
mod update_console_history;
mod spawn_collision_boxes;
mod show_info_info_system;
mod show_info_attribute_value_system;
mod update_info_fps_system;
mod show_info_gun_attribute_value_system;

pub struct CmdUiPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CmdUiEnterConsoleSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CmdUiExitConsoleSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CmdUiUpdateConsoleSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CmdUiFixedUpdateConsoleSystemSet;


impl Plugin for CmdUiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            CmdUiEnterConsoleSystemSet
                .in_base_set(BaseSets::Update)
            // .in_schedule(OnEnter(ConsoleState::Shown))
        );

        app.configure_set(
            CmdUiExitConsoleSystemSet
                .in_base_set(BaseSets::Update)
            // .in_schedule(OnExit(ConsoleState::Shown))
        );

        app.configure_set(
            CmdUiUpdateConsoleSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(in_state(ConsoleState::Shown))
        );

        app.configure_set(
            CmdUiFixedUpdateConsoleSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(in_state(ConsoleState::Shown))
                .run_if(on_fixed_timer(Duration::from_millis(1000)))
        );

        app.add_system(init_collision_boxes.in_schedule(OnEnter(ConsoleState::Shown)));

        app.add_system(despawn_recursive_system::<CollisionBox>.in_schedule(OnExit(ConsoleState::Shown)));

        app
            .add_system(init_new_collision_boxes.in_set(CmdUiUpdateConsoleSystemSet))
            .add_system(update_console_history.in_set(CmdUiUpdateConsoleSystemSet))
            .add_system(show_debug_window_system.in_set(CmdUiUpdateConsoleSystemSet))
            .add_system(show_info_window_system.in_set(CmdUiUpdateConsoleSystemSet));
        //.add_system(show_quad_tree_system.in_set(CmdUiUpdateConsoleSystemSet))

        app
            .add_system(show_info_fps_system.in_set(CmdUiFixedUpdateConsoleSystemSet))
            .add_system(update_info_attribute_value_system::<Health>.in_set(CmdUiFixedUpdateConsoleSystemSet))
            .add_system(update_info_attribute_value_system::<Damage>.in_set(CmdUiFixedUpdateConsoleSystemSet))
            .add_system(update_info_attribute_value_system::<UnitSize>.in_set(CmdUiFixedUpdateConsoleSystemSet))
            .add_system(update_info_attribute_value_system::<MoveSpeed>.in_set(CmdUiFixedUpdateConsoleSystemSet))
            .add_system(update_info_attribute_value_system::<Reload>.in_set(CmdUiFixedUpdateConsoleSystemSet))
            .add_system(update_info_gun_attribute_value_system::<Damage>.in_set(CmdUiFixedUpdateConsoleSystemSet));
    }
}
