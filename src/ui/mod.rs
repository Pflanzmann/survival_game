use bevy::prelude::{App, Plugin, SystemSet};
use crate::AppState;
use crate::ui::game_over_screen::{button_click_system, spawn_menu_system};
use crate::ui::hud_system::{spawn_text_system, update_text_system};

pub mod game_over_screen;
pub mod hud_system;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_click_system)
            .add_system(update_text_system)

            .add_startup_system(spawn_text_system)

            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
                    .with_system(spawn_menu_system)
            );
    }
}