use bevy::prelude::Resource;
use serde::Deserialize;

use crate::models::spawner::spawn_phase::SpawnPhase;

#[derive(Default, Deserialize, Resource)]
pub struct SpawnStage {
    pub spawn_phases: Vec<SpawnPhase>,
}