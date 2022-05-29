use bevy::prelude::{App, Plugin};
use crate::world::background::BackgroundPlugin;
use crate::world::drops::DropsPlugin;
use crate::world::spawner::SpawnerPlugin;

pub mod background;
pub mod drops;
pub mod spawner;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(DropsPlugin)
            .add_plugin(BackgroundPlugin)
            .add_plugin(SpawnerPlugin)
        ;
    }
}
