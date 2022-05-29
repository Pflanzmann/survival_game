use bevy::prelude::{App, Plugin, SystemSet};

use crate::AppState;
use crate::util::stage_label_helper::in_update;
use crate::world::background::BackgroundPlugin;
use crate::world::drops::DropsPlugin;
use crate::world::game_time_system::game_time_system;
use crate::world::spawner::SpawnerPlugin;

pub mod background;
pub mod drops;
pub mod spawner;
mod game_time_system;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(DropsPlugin)
            .add_plugin(BackgroundPlugin)
            .add_plugin(SpawnerPlugin)

            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(game_time_system)
                )
            );
    }
}
