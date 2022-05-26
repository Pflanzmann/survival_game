use bevy::prelude::{EventReader, EventWriter, Query, With};

use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::models::events::debug_command_info_event::DebugCommandInfoEvent;
use crate::models::gold_storage::GoldStorage;
use crate::models::player::Player;

const KEY: &str = "gold";

pub fn add_gold_command(
    mut debug_command_events: EventReader<DebugCommandEvent>,
    mut debug_command_info_event: EventWriter<DebugCommandInfoEvent>,
    mut player_query: Query<&mut GoldStorage, With<Player>>,
) {
    for debug_command_event in debug_command_events.iter() {
        if debug_command_event.key != KEY {
            continue;
        }

        let amount_string = match debug_command_event.values.first() {
            Some(amount_string) => amount_string,
            None => {
                debug_command_info_event.send(DebugCommandInfoEvent { debug_command: "No values passed.".to_string() });
                continue;
            }
        };

        let amount: i32 = match amount_string.parse::<i32>() {
            Ok(amount) => amount,
            Err(_) => {
                debug_command_info_event.send(DebugCommandInfoEvent { debug_command: "The passed value is not a i32.".to_string() });
                continue;
            }
        };

        let mut player_gold = match player_query.get_single_mut() {
            Ok(player_entity) => player_entity,
            Err(_) => {
                debug_command_info_event.send(DebugCommandInfoEvent { debug_command: "Could not find a player to add gold.".to_string() });
                continue;
            }
        };

        player_gold.number += amount;
        debug_command_info_event.send(DebugCommandInfoEvent { debug_command: format!("Successfully added {} amount of gold to the player.", amount) });
    }
}