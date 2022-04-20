use bevy::prelude::{Entity, EventReader, EventWriter, Query, With, Without};

use crate::models::enemy::Enemy;
use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::models::events::debug_command_info_event::DebugCommandInfoEvent;
use crate::models::events::remove_mod_from_target_event::RemoveModFromTargetEvent;
use crate::models::modifications::descriptors::mod_name::ModName;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::player::Player;

const KEY: &str = "remove";

pub fn remove_mod_command(
    mut debug_command_events: EventReader<DebugCommandEvent>,
    mut debug_command_info_event: EventWriter<DebugCommandInfoEvent>,
    mut apply_event: EventWriter<RemoveModFromTargetEvent>,
    mod_entities: Query<(Entity, &ModName), (With<Modification>, Without<Player>, Without<Enemy>)>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
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
            None => {
                debug_command_info_event.send(DebugCommandInfoEvent { debug_command: format!("Could not find mod with name [{}]", typed_mod_name) });
                continue;
            }
        };

        let mut counter = 0;
        match target.as_str() {
            "-p" | "player" => {
                for player_entity in player_query.iter() {
                    apply_event.send(RemoveModFromTargetEvent { mod_entity: chosen_mod, target_entity: player_entity });
                    counter += 1;
                }
            }

            "-e" | "enemies" => {
                for entity_entity in enemy_query.iter() {
                    apply_event.send(RemoveModFromTargetEvent { mod_entity: chosen_mod, target_entity: entity_entity });
                    counter += 1;
                }
            }
            _ => {
                debug_command_info_event.send(DebugCommandInfoEvent { debug_command: "Invalid target parameter".to_string() });
                continue;
            }
        }

        debug_command_info_event.send(DebugCommandInfoEvent { debug_command: format!("Did remove [{}] from {} targets", typed_mod_name, counter) });
    }
}
