use bevy::core::FixedTimestep;
use bevy::prelude::{App, Plugin, SystemSet};

use crate::AppState;
use crate::ui::cmd::CmdUiPlugin;
use crate::ui::fps_counter_update_system::fps_counter_update_system;
use crate::ui::game_over_screen::{button_click_system, spawn_menu_system};
use crate::ui::hud_system::{spawn_text_system, update_bullet_hud_system, update_text_system};
use crate::ui::main_menu_screen::{close_main_menu_system, spawn_main_menu_system};
use crate::ui::pause_screen::{enter_pause_system, exit_pause_system};
use crate::ui::shop_system::{close_shop_menu_system, shop_button_system, spawn_shop_menu_system};
use crate::util::stage_label_helper::{in_last, in_update};

mod game_over_screen;
mod hud_system;
mod pause_screen;
mod main_menu_screen;
mod shop_system;
mod cmd;
mod fps_counter_update_system;

/// This plugin generates the [ UI ]  elements for game menus and
/// the ingame hud. Furthermore it holds systems to control the
/// spawning and despawning of UI elements as well as interactions
/// like button clicks.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(CmdUiPlugin)

            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(update_text_system)
                        .with_system(update_bullet_hud_system)
                )
            )

            .add_system_set(
                in_last(
                    SystemSet::on_update(AppState::Shop)
                        .with_system(update_bullet_hud_system)
                )
            )

            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_run_criteria(FixedTimestep::step(0.1))
                        .with_system(fps_counter_update_system)
                )
            )

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

            .add_system(button_click_system)
            .add_system(shop_button_system);
    }
}