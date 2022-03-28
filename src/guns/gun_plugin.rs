use bevy::prelude::Plugin;
use crate::App;
use crate::guns::basic_gun_system::basic_gun_system;

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(basic_gun_system);
    }
}