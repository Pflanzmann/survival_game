use bevy::prelude::{Commands, ResMut};
use crate::models::resources::world::spawn_phase_timer::SpawnStageState;

use crate::models::spawner::spawn_stage::SpawnStage;
use crate::util::read_file_to_string::read_file_to_string;

pub fn preload_stage_spawn_behavior_system(
    mut commands: Commands,
    mut spawn_phase_timer: ResMut<SpawnStageState>,
) {
    let my_string = read_file_to_string("configurations/stage_spawn_behaviors/default_spawn_phase.json");
    let spawn_stage_behavior: SpawnStage = serde_json::from_str(&my_string).expect("JSON was not well-formatted");
    
    spawn_phase_timer.phase_timer = spawn_stage_behavior.spawn_phases[0].duration;

    commands.insert_resource(spawn_stage_behavior);
}
