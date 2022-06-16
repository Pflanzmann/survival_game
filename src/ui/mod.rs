use bevy::app::IntoSystemAppConfig;
use bevy::prelude::*;

use crate::AppState;
use crate::scheduling::BaseSets;
use crate::ui::cmd::CmdUiPlugin;
use crate::ui::main_menu_screen::show_main_menu_system;
use crate::ui::pause_screen::{close_pause_system, spawn_pause_system};
use crate::ui::setup_tool_tip_window::{move_tool_tip_window, populate_tooltip_window, setup_tool_tip_window};
use crate::ui::shop_system::{close_shop_menu_system, shop_button_system, spawn_shop_menu_system};
use crate::ui::update::update_hud_state::update_hud_state;
use crate::ui::views::show_game_over_system::show_game_over_system;
use crate::ui::views::show_game_won_system::show_game_won_system;
use crate::ui::views::show_hud_system::show_hud_system;

mod pause_screen;
mod shop_system;
mod cmd;
mod setup_tool_tip_window;
mod views;
mod update;
mod main_menu_screen;

/// This plugin generates the UI elements for game menus and
/// the ingame hud. Furthermore it holds systems to control the
/// spawning and despawning of UI elements as well as interactions
/// like button clicks.
pub struct UiPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UiUpdateSystemSet;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CmdUiPlugin);

        app.configure_set(
            UiUpdateSystemSet
                .in_base_set(BaseSets::Update)
        );

        app
            // .add_system(spawn_hud_system.in_schedule(OnExit(AppState::MainMenu)))
            .add_system(spawn_pause_system.in_schedule(OnEnter(AppState::Paused)))
            .add_system(close_pause_system.in_schedule(OnExit(AppState::Paused)))
            .add_system(spawn_shop_menu_system.in_schedule(OnEnter(AppState::Shop)))
            .add_system(close_shop_menu_system.in_schedule(OnExit(AppState::Shop)));

        app
            .add_system(shop_button_system.in_set(UiUpdateSystemSet))
            // .add_system(update_gold_text_system.in_set(UiUpdateSystemSet))

            .add_system(show_main_menu_system.run_if(in_state(AppState::MainMenu)))
            .add_system(show_game_over_system.run_if(in_state(AppState::GameOver)))
            .add_system(show_game_won_system.run_if(in_state(AppState::GameWon)))

            .add_system(show_hud_system.in_set(UiUpdateSystemSet))
            .add_system(update_hud_state.in_set(UiUpdateSystemSet))

            .add_system(move_tool_tip_window.in_set(UiUpdateSystemSet))
            .add_system(setup_tool_tip_window.in_set(UiUpdateSystemSet))
            .add_system(populate_tooltip_window.in_set(UiUpdateSystemSet));
    }
}