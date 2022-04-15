use bevy::prelude::{Plugin, SystemSet};

use crate::{App, AppState};
use crate::spawner::spawn_scheduler_system::spawn_scheduler_system;
use crate::spawner::spawn_worker_system::spawn_worker_system;
use crate::util::stage_label_helper::in_update;

mod spawn_scheduler_system;
mod spawn_worker_system;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(spawn_scheduler_system)
                        .with_system(spawn_worker_system)
                )
            );
    }
}