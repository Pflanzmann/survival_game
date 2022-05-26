use bevy::prelude::Plugin;

use crate::App;

pub mod show_quad_tree_system;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, _app: &mut App) {}
}
