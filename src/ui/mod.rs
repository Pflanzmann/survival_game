use bevy::app::IntoSystemAppConfig;
use bevy::prelude::*;

use crate::AppState;
use crate::scheduling::BaseSets;
use crate::ui::debug::CmdUiPlugin;
use crate::ui::main_menu::MainMenuPlugin;
use crate::ui::pause_screen::{close_pause_system, spawn_pause_system};
use crate::ui::update::update_hud_state::update_hud_state;
use crate::ui::update::update_shop_state::update_shop_system;
use crate::ui::views::show_game_over_system::show_game_over_system;
use crate::ui::views::show_game_won_system::show_game_won_system;
use crate::ui::views::show_hud_system::show_hud_system;
use crate::ui::views::show_shop_system::show_shop_system;

mod pause_screen;
mod views;
mod update;
mod debug;
mod main_menu;

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
        app.add_plugin(MainMenuPlugin);

        app.configure_set(
            UiUpdateSystemSet
                .in_base_set(BaseSets::Update)
        );

        app
            .add_system(spawn_pause_system.in_schedule(OnEnter(AppState::Paused)))
            .add_system(close_pause_system.in_schedule(OnExit(AppState::Paused)));

        app
            .add_system(update_shop_system.in_schedule(OnEnter(AppState::Shop)))
            .add_system(show_shop_system.in_set(UiUpdateSystemSet).run_if(in_state(AppState::Shop)));

        app
            .add_system(show_game_over_system.in_set(UiUpdateSystemSet).run_if(in_state(AppState::GameOver)))
            .add_system(show_game_won_system.in_set(UiUpdateSystemSet).run_if(in_state(AppState::GameWon)))

            .add_system(show_hud_system.in_set(UiUpdateSystemSet).run_if(in_state(AppState::InGame)))
            .add_system(show_hud_system.in_set(UiUpdateSystemSet).run_if(in_state(AppState::Shop)))
            .add_system(show_hud_system.in_set(UiUpdateSystemSet).run_if(in_state(AppState::Paused)))
            .add_system(update_hud_state.in_set(UiUpdateSystemSet).run_if(in_state(AppState::InGame)));
    }
}