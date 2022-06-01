use bevy::prelude::{App, Plugin, SystemSet};

use crate::AppState;
use crate::models::ui::game_won_screen::GameWonScreen;
use crate::ui::cmd::CmdUiPlugin;
use crate::ui::game_over_screen::{button_click_system, spawn_menu_system};
use crate::ui::game_won_screen::spawn_game_won_screen_system;
use crate::ui::hud_system::{spawn_text_system, update_bullet_hud_system, update_text_system};
use crate::ui::main_menu_screen::{close_main_menu_system, spawn_main_menu_system};
use crate::ui::pause_screen::{enter_pause_system, exit_pause_system};
use crate::ui::setup_tool_tip_window::{move_tool_tip_window, populate_tooltip_window, setup_tool_tip_window};
use crate::ui::shop_system::{close_shop_menu_system, shop_button_system, spawn_shop_menu_system};
use crate::util::helper_systems::despawn_recursive_system::despawn_recursive_system;

mod game_over_screen;
mod hud_system;
mod pause_screen;
mod main_menu_screen;
mod shop_system;
mod cmd;
mod setup_tool_tip_window;
mod game_won_screen;

/// This plugin generates the UI elements for game menus and
/// the ingame hud. Furthermore it holds systems to control the
/// spawning and despawning of UI elements as well as interactions
/// like button clicks.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(CmdUiPlugin)

            .add_system_set(
                SystemSet::on_exit(AppState::MainMenu)
                    .with_system(spawn_text_system)
            )

            .add_system_set(
                SystemSet::on_enter(AppState::GameOver)
                    .with_system(spawn_menu_system)
            )

            .add_system_set(
                SystemSet::on_enter(AppState::Paused)
                    .with_system(enter_pause_system)
            )

            .add_system_set(
                SystemSet::on_exit(AppState::Paused)
                    .with_system(exit_pause_system)
            )

            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
                    .with_system(spawn_main_menu_system)
            )

            .add_system_set(
                SystemSet::on_exit(AppState::MainMenu)
                    .with_system(close_main_menu_system)
            )

            .add_system_set(
                SystemSet::on_enter(AppState::Shop)
                    .with_system(spawn_shop_menu_system)
            )

            .add_system_set(
                SystemSet::on_exit(AppState::Shop)
                    .with_system(close_shop_menu_system)
            )

            .add_system_set(
                SystemSet::on_enter(AppState::GameWon)
                    .with_system(spawn_game_won_screen_system)
            )

            .add_system_set(
                SystemSet::on_exit(AppState::GameWon)
                    .with_system(despawn_recursive_system::<GameWonScreen>)
            )

            .add_system(button_click_system)
            .add_system(shop_button_system)
            .add_system(update_text_system)
            .add_system(setup_tool_tip_window)
            .add_system(move_tool_tip_window)
            .add_system(populate_tooltip_window)
            .add_system(update_bullet_hud_system);
    }
}