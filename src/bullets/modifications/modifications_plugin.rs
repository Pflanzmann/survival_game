use bevy::prelude::Plugin;

use crate::App;
use crate::bullets::modifications::curve_shot_system::{apply_curved_shot_system, curve_shot_system};
use crate::bullets::modifications::grow_shot_system::{apply_grow_shot_system, grow_shot_system};
use crate::bullets::modifications::split_shot_system::{apply_split_shot_system, split_shot_system};

pub struct ModificationsPlugin;

impl Plugin for ModificationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(curve_shot_system)
            .add_system(apply_curved_shot_system)

            .add_system(apply_grow_shot_system)
            .add_system(grow_shot_system)

            .add_system(apply_split_shot_system)
            .add_system(split_shot_system);
    }
}