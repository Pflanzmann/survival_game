use bevy::prelude::*;

use crate::{App, AppState};
use crate::scheduling::BaseSets;
use crate::world::spawner::spawn_scheduler_system::spawn_scheduler_system;
use crate::world::spawner::spawn_worker_system::spawn_worker_system;

mod spawn_scheduler_system;
mod spawn_worker_system;

pub struct SpawnerPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SpawnerSystemSet;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            SpawnerSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(in_state(AppState::InGame))
        );

        app
            .add_system(spawn_scheduler_system.in_set(SpawnerSystemSet))
            .add_system(spawn_worker_system.in_set(SpawnerSystemSet));
    }
}