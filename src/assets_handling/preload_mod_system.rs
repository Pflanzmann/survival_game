use std::fs;

use bevy::prelude::{BuildChildren, Commands, Name, NonSend};

use crate::util::entity_builder::EntityBuilder;

pub fn preload_mod_system(
    mut commands: Commands,
    entity_builder: NonSend<EntityBuilder>,
) {
    let parent = commands.spawn().insert(Name::new("Mod Entities")).id();

    let base_path = "configurations/mods/";
    let paths = fs::read_dir(base_path).unwrap();

    for path in paths {
        let mut file_path = String::new();
        file_path.push_str(base_path);

        let child = entity_builder.spawn_entity(&mut commands, path.unwrap().path().display().to_string().as_str());
        commands.entity(parent).add_child(child);
    }
}
