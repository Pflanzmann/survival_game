use bevy::core::FixedTimestep;
use bevy::prelude::{App, Plugin, SystemSet};

use crate::components::event_components::EnemyDiedEvent;
use crate::drops::basic_drop_system::basic_drop_system;

pub struct DropsPlugin;

impl Plugin for DropsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(basic_drop_system);
    }
}