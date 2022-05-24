use serde::Deserialize;

use crate::models::spawner::spawn_phase::SpawnPhase;

#[derive(Default, Deserialize)]
pub struct StageSpawnBehavior {
    pub spawn_phases: Vec<SpawnPhase>,
}