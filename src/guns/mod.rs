use bevy::prelude::{Plugin, SystemSet};

use crate::{App, AppState};
use crate::guns::gun_reloading_timer_system::gun_reloading_timer_system;
use crate::guns::setup_basic_gun_system::setup_basic_gun_system;
use crate::guns::straight_basic_shot_system::straight_basic_shot_system;
use crate::util::stage_label_helper::in_update;

mod straight_basic_shot_system;
mod setup_basic_gun_system;
mod gun_reloading_timer_system;

pub struct GunPlugin;

/// Plugin to handle the gun and shooting systems.
///
/// The [straight_basic_shot_system] is a shooting system to trigger the creation of [Bullet]s.
/// Other systems get called in the update of the [AppState::InGame].
///
/// [setup_gun_system] is called in the exit of the [AppState::MainMenu] for now.
impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_update(
                    SystemSet::new()
                        .with_system(setup_basic_gun_system)
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