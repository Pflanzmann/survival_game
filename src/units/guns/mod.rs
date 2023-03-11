use bevy::prelude::*;

use crate::{App, AppState};
use crate::scheduling::BaseSets;
use crate::units::guns::basic_sword_system::basic_sword_system;
use crate::units::guns::gun_reloading_timer_system::gun_reloading_timer_system;
use crate::units::guns::setup_basic_gun_system::setup_basic_gun_system;
use crate::units::guns::setup_sword_system::setup_sword_system;
use crate::units::guns::straight_basic_shot_system::straight_basic_shot_system;

mod straight_basic_shot_system;
mod setup_basic_gun_system;
mod gun_reloading_timer_system;
mod setup_sword_system;
mod basic_sword_system;

/// Plugin to handle the gun and shooting systems.
///
/// The [straight_basic_shot_system] is a shooting system to trigger the creation of [Projectile]s.
/// Other systems get called in the update of the [AppState::InGame].
///
/// [setup_gun_system] is called in the exit of the [AppState::MainMenu] for now.
pub struct GunPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct GunSystemSet;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            GunSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(in_state(AppState::InGame))
        );

        app
            .add_system(setup_basic_gun_system.in_set(GunSystemSet))
            .add_system(setup_sword_system.in_set(GunSystemSet))
            .add_system(straight_basic_shot_system.in_set(GunSystemSet))
            .add_system(basic_sword_system.in_set(GunSystemSet))
            .add_system(gun_reloading_timer_system.in_set(GunSystemSet));
    }
}
