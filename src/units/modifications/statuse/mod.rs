use bevy::prelude::{Plugin, SystemSet};

use crate::{App, AppState};
use crate::units::modifications::statuse::burn_system::burn_system;

mod burn_system;
mod helper;

pub struct StatusPlugin;

impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(burn_system)
            )
        ;
    }
}
