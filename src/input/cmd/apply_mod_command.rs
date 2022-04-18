use bevy::prelude::{Entity, EventReader, EventWriter, Query, With, Without};

use crate::models::enemy::Enemy;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::models::modifications::descriptors::mod_name::ModName;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::player::Player;

const KEY: &str = "apply";

pub fn apply_mod_command(
    mut debug_command_events: EventReader<DebugCommandEvent>,
    mut apply_event: EventWriter<ApplyModToTargetEvent>,
    mod_entities: Query<(Entity, &ModName), (With<Modification>, Without<Player>, Without<Enemy>)>,
    player_query: Query<Entity, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<Entity, (With<Enemy>, Without<Player>)>,
) {
    for debug_command_event in debug_command_events.iter() {
        let command: String = debug_command_event.debug_command.clone();
        let mut arguments = command.split_whitespace();

        let key = match arguments.next() {
            Some(value) => value.to_lowercase(),
            None => continue,
        };

        if key != KEY {
            continue;
        }

        let mut arguments_list: Vec<&str> = arguments.collect();

        let target = match arguments_list.last() {
            Some(value) => value.to_lowercase(),
            None => continue,
        };
        arguments_list.pop();

        let mut typed_mod_name = String::new();
        for text in arguments_list.iter() {
            typed_mod_name.push_str(text);
            typed_mod_name.push(' ');
        }
        typed_mod_name = typed_mod_name.to_lowercase().trim().to_string();

        let mut chosen_mod: Option<Entity> = Option::None;
        for (mod_entity, mod_name) in mod_entities.iter() {
            if mod_name.mod_name.to_lowercase() == typed_mod_name {
                chosen_mod = Some(mod_entity)
            }
        }

        let chosen_mod = match chosen_mod {
            Some(value) => value,
            None => continue,
        };

        match target.as_str() {
            "player" => {
                for player_entity in player_query.iter() {
                    apply_event.send(ApplyModToTargetEvent { mod_entity: chosen_mod, target_entity: player_entity })
                }
            }

            "enemies" => {
                for entity_entity in enemy_query.iter() {
                    apply_event.send(ApplyModToTargetEvent { mod_entity: chosen_mod, target_entity: entity_entity })
                }
            }

            _ => continue,
        }
    }
}