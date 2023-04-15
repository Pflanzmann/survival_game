use bevy::prelude::Resource;
use serde::Deserialize;

use crate::models::spawner::spawn_phase::SpawnPhase;

#[derive(Default, Deserialize, Resource, Clone)]
pub struct Stage {
    pub name: String,
    pub spawn_phases: Vec<SpawnPhase>,
}