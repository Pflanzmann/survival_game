use bevy::core::FixedTimestep;
use bevy::prelude::{App, Plugin, SystemSet};

use crate::drops::basic_drop_system::basic_drop_system;
use crate::components::event_components::EnemyDiedEvent;

pub struct DropsPlugin;

impl Plugin for DropsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(basic_drop_system);
    }
}