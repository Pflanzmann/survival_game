use bevy::prelude::*;

use crate::AppState;
use crate::scheduling::BaseSets;
use crate::world::background::BackgroundPlugin;
use crate::world::drops::DropsPlugin;
use crate::world::game_time_system::game_time_system;
use crate::world::spawner::SpawnerPlugin;

pub mod background;
pub mod drops;
pub mod spawner;
mod game_time_system;

pub struct WorldPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct WorldSystemSet;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            WorldSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(in_state(AppState::InGame))
        );

        app
            .add_plugin(DropsPlugin)
            .add_plugin(BackgroundPlugin)
            .add_plugin(SpawnerPlugin);

        app.add_system(game_time_system.in_set(WorldSystemSet));
    }
}
