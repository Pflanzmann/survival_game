use bevy::core::FixedTimestep;
use bevy::prelude::{Plugin, SystemSet};

use crate::App;
use crate::collision::collision_detection_system::collision_detection_system;

pub struct CollisionPlugin;

const FIXED_TIMESTEP: f64 = 15.0 / 60.0;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(FIXED_TIMESTEP))
                .with_system(collision_detection_system)
        );
    }
}