use bevy::app::{App, Plugin};
use bevy::prelude::*;
use crate::{AppState, MainMenuState};
use crate::scheduling::BaseSets;
use crate::ui::main_menu::show_choose_stage_system::show_choose_stage_system;
use crate::ui::main_menu::show_title_screen_system::show_title_screen_system;
use crate::ui::main_menu::show_main_menu_system::show_main_menu_system;
use crate::ui::main_menu::show_options_system::show_options_system;

mod show_title_screen_system;
mod show_main_menu_system;
mod show_options_system;
mod show_choose_stage_system;

pub struct MainMenuPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MainMenuSystemSet;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            MainMenuSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(in_state(AppState::MainMenu))
        );

        app
            .add_system(show_title_screen_system.in_set(MainMenuSystemSet).run_if(in_state(MainMenuState::Entry)))
            .add_system(show_main_menu_system.in_set(MainMenuSystemSet).run_if(in_state(MainMenuState::MainMenu)))
            .add_system(show_choose_stage_system.in_set(MainMenuSystemSet).run_if(in_state(MainMenuState::ChooseStage)))
            .add_system(show_options_system.in_set(MainMenuSystemSet).run_if(in_state(MainMenuState::Options)));
    }
}
