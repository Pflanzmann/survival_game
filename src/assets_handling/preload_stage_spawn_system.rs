use bevy::prelude::{ResMut, Resource};
use crate::models::resources::world::spawn_phase_timer::SpawnPhaseTimer;

use crate::models::spawner::stage_spawn_behavior::StageSpawnBehavior;
use crate::util::read_file_to_string::read_file_to_string;

#[derive(Default, Resource)]
pub struct StageSpawnBehaviorHandle {
    pub default_spawn_phase: StageSpawnBehavior,
}

pub fn preload_stage_spawn_behavior_system(
    mut stage_spawn_behavior_handle: ResMut<StageSpawnBehaviorHandle>,
    mut spawn_phase_timer: ResMut<SpawnPhaseTimer>,
) {
    let my_string = read_file_to_string("configurations/stage_spawn_behaviors/default_spawn_phase.json");
    stage_spawn_behavior_handle.default_spawn_phase = serde_json::from_str(&my_string).expect("JSON was not well-formatted");

    spawn_phase_timer.timer = stage_spawn_behavior_handle.default_spawn_phase.spawn_phases[0].duration
}
