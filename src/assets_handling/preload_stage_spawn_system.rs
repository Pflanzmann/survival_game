use std::fs;

use bevy::prelude::*;

use crate::models::spawner::spawn_stage::Stage;
use crate::util::read_file_to_string::read_file_to_string;

#[derive(Default, Resource)]
pub struct StageList {
    pub stages: Vec<Stage>,
}

pub fn preload_stage_spawn_behavior_system(
    mut commands: Commands,
) {
    let base_path = "configurations/stages/";
    let paths = fs::read_dir(base_path).unwrap();

    let mut stage_list = StageList::default();

    for path in paths {
        let my_string = read_file_to_string(path.unwrap().path().display().to_string().as_str());
        let stage: Stage = serde_json::from_str(&my_string).expect("JSON was not well-formatted");
        stage_list.stages.push(stage)
    }

    commands.insert_resource(stage_list);
}
