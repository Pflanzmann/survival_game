use bevy::prelude::{BuildChildren, Commands, Entity, Name, NonSend, Res};

use crate::util::entity_builder::EntityBuilder;

pub fn preload_mod_system(
    mut commands: Commands,
    mut entity_builder: NonSend<EntityBuilder>,
) {
    let mut children: Vec<Entity> = Vec::new();

    children.push(entity_builder.spawn_entity(&mut commands, "configurations/mod_configurations/grow_shot_config.json"));
    children.push(entity_builder.spawn_entity(&mut commands, "configurations/mod_configurations/curve_shot_config.json"));
    children.push(entity_builder.spawn_entity(&mut commands, "configurations/mod_configurations/split_shot_config.json"));
    children.push(entity_builder.spawn_entity(&mut commands, "configurations/mod_configurations/sprinting_config.json"));

    let parent = commands.spawn().insert(Name::new("Mod Entities")).id();
    commands.entity(parent).push_children(&*children);
}
