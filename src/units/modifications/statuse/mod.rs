use bevy::prelude::*;

use crate::{App, AppState};
use crate::scheduling::BaseSets;
use crate::units::modifications::statuse::burn_system::burn_system;
use crate::units::modifications::statuse::magnet_system::magnet_system;

mod burn_system;
mod helper;
pub mod magnet_system;

pub struct StatusPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct StatusSystemSet;

impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            StatusSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(in_state(AppState::InGame))
        );

        app
            .add_system(burn_system.in_set(StatusSystemSet))
            .add_system(magnet_system.in_set(StatusSystemSet));
    }
}
