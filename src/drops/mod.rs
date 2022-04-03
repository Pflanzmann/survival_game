use bevy::prelude::{App, Plugin};

use crate::drops::basic_drop_system::basic_drop_system;

pub mod basic_drop_system;

pub struct DropsPlugin;

impl Plugin for DropsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(basic_drop_system);
    }
}