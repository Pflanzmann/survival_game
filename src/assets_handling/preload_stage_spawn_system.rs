use bevy::prelude::ResMut;

use crate::models::spawner::stage_spawn_behavior::StageSpawnBehavior;
use crate::util::read_file_to_string::read_file_to_string;

#[derive(Default)]
pub struct StageSpawnBehaviorHandle {
    pub default_spawn_phase: StageSpawnBehavior,
}

pub fn preload_stage_spawn_behvaior_system(
    mut stage_spawn_behavior_handle: ResMut<StageSpawnBehaviorHandle>,
) {
    let my_string = read_file_to_string("configurations/stage_spawn_behaviors/default_spawn_phase.json");
    stage_spawn_behavior_handle.default_spawn_phase = serde_json::from_str(&my_string).expect("JSON was not well-formatted");
}
