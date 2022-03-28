use bevy::prelude::Plugin;
use crate::App;
use crate::bullets::bullet_movement_system::bullet_movement_system;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bullet_movement_system);
    }
}