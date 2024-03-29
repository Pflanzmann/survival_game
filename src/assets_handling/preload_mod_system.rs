use std::fs;

use bevy::prelude::{AssetServer, BuildChildren, Commands, Name, Res};

use crate::util::entity_builder::EntityBuilder;

pub fn preload_mod_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    entity_builder: Res<EntityBuilder>,
) {
    let parent = commands.spawn_empty().insert(Name::new("Mod Entities")).id();

    let base_path = "configurations/mods/";
    let paths = fs::read_dir(base_path).unwrap();

    for path in paths {
        let child = entity_builder.spawn_entity(&mut commands, &asset_server, path.unwrap().path().display().to_string().as_str());
        commands.entity(parent).add_child(child);
    }
}
