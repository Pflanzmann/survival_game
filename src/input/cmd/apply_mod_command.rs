use bevy::prelude::{Entity, EventReader, EventWriter, Query, With};

use crate::models::enemy::Enemy;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::models::events::debug_command_info_event::DebugCommandInfoEvent;
use crate::models::modifications::descriptors::mod_name::ModName;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::player::Player;

const KEY: &str = "apply";

pub fn apply_mod_command(
    mut debug_command_events: EventReader<DebugCommandEvent>,
    mut debug_command_info_event: EventWriter<DebugCommandInfoEvent>,
    mut apply_event: EventWriter<ApplyModToTargetEvent>,
    mod_entities: Query<(Entity, &ModName), With<Modification>>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for debug_command_event in debug_command_events.iter() {
        if debug_command_event.key != KEY {
            continue;
        }

        for typed_mod_name in debug_command_event.values.iter() {
            let mut chosen_mod: Option<Entity> = Option::None;
            for (mod_entity, mod_name) in mod_entities.iter() {
                if mod_name.mod_name.to_lowercase().replace(" ", "") == *typed_mod_name {
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

            for target in debug_command_event.arguments.iter() {
                let mut counter = 0;
                match target.as_str() {
                    "-p" | "player" => {
                        for player_entity in player_query.iter() {
                            apply_event.send(ApplyModToTargetEvent { mod_entity: chosen_mod, target_entity: player_entity });
                            counter += 1;
                        }
                    }

                    "-e" | "enemies" => {
                        for entity_entity in enemy_query.iter() {
                            apply_event.send(ApplyModToTargetEvent { mod_entity: chosen_mod, target_entity: entity_entity });
                            counter += 1;
                        }
                    }
                    _ => {
                        debug_command_info_event.send(DebugCommandInfoEvent { debug_command: format!("Invalid target parameter [{}]", target) });
                        continue;
                    }
                }

                debug_command_info_event.send(DebugCommandInfoEvent { debug_command: format!("Did apply [{}] for {} targets", typed_mod_name, counter) });
            }
        }
    }
}