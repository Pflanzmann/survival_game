use bevy::prelude::{BuildChildren, Commands, Entity, Name, NonSend};

use crate::util::entity_builder::EntityBuilder;

pub fn preload_mod_system(
    mut commands: Commands,
    entity_builder: NonSend<EntityBuilder>,
) {
    let children: Vec<Entity> = vec![
        entity_builder.spawn_entity(&mut commands, "configurations/mod_configurations/grow_shot_config.json"),
        entity_builder.spawn_entity(&mut commands, "configurations/mod_configurations/curve_shot_config.json"),
        entity_builder.spawn_entity(&mut commands, "configurations/mod_configurations/split_shot_config.json"),
        entity_builder.spawn_entity(&mut commands, "configurations/mod_configurations/sprinting_config.json"),
        entity_builder.spawn_entity(&mut commands, "configurations/mod_configurations/turret_config.json"),
        entity_builder.spawn_entity(&mut commands, "configurations/mod_configurations/slime_config.json"),
        entity_builder.spawn_entity(&mut commands, "configurations/mod_configurations/death_ball.json"),
        entity_builder.spawn_entity(&mut commands, "configurations/mod_configurations/psy_rock.json"),
    ];

    let parent = commands.spawn().insert(Name::new("Mod Entities")).id();
    commands.entity(parent).push_children(&*children);
}
