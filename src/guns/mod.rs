use bevy::prelude::Plugin;

use crate::{App, AppState, SystemSet};
use crate::guns::gun_reloading_timer_system::gun_reloading_timer_system;
use crate::guns::setup_gun_system::setup_gun_system;
use crate::guns::straight_basic_shot_system::straight_basic_shot_system;
use crate::util::stage_label_helper::in_update;

mod straight_basic_shot_system;
mod setup_gun_system;
mod gun_reloading_timer_system;

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_update(
                    SystemSet::on_exit(AppState::MainMenu)
                        .with_system(setup_gun_system)
                )
            )

            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(straight_basic_shot_system)
                        .with_system(gun_reloading_timer_system)
                )
            );
    }
}