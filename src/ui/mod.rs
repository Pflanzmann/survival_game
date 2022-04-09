use bevy::prelude::{App, Plugin, SystemSet};
use crate::AppState;
use crate::ui::game_over_screen::{button_click_system, spawn_menu_system};
use crate::ui::hud_system::{spawn_text_system, update_bullet_hud_system, update_text_system};
use crate::ui::pause_screen::{enter_pause_system, exit_pause_system};

pub mod game_over_screen;
pub mod hud_system;
pub mod pause_screen;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_system_set(SystemSet::on_update(AppState::InGame)
                .with_system(update_text_system)
                .with_system(update_bullet_hud_system)
            )

            .add_system_set(SystemSet::on_enter(AppState::InGame)
                .with_system(spawn_text_system)
            )

            .add_system_set(SystemSet::on_enter(AppState::GameOver)
                    .with_system(spawn_menu_system)
            )

           .add_system_set(SystemSet::on_enter(AppState::Paused)
               .with_system(enter_pause_system)
           )

           .add_system_set(SystemSet::on_exit(AppState::Paused)
               .with_system(exit_pause_system)
           )

            .add_system(button_click_system);
    }
}