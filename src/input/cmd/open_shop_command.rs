use bevy::prelude::{Entity, EventReader, EventWriter, NextState, Query, ResMut, With};

use crate::AppState;
use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::models::events::debug_command_info_event::DebugCommandInfoEvent;
use crate::models::player::Player;
use crate::models::resources::shop_customer::ShopCustomer;

const KEY: &str = "shop";

pub fn open_shop_command(
    mut debug_command_events: EventReader<DebugCommandEvent>,
    mut debug_command_info_event: EventWriter<DebugCommandInfoEvent>,
    mut shop_customer: ResMut<ShopCustomer>,
    mut next_state: ResMut<NextState<AppState>>,
    player_query: Query<Entity, With<Player>>,
) {
    for debug_command_event in debug_command_events.iter() {
        if debug_command_event.key != KEY {
            continue;
        }

        match debug_command_event.values.first() {
            Some(value) => {
                if value != "open" {
                    debug_command_info_event.send(DebugCommandInfoEvent { debug_command: format!("Invalid [shop] value: {}", value) });
                    continue;
                }
            }
            None => {
                debug_command_info_event.send(DebugCommandInfoEvent { debug_command: "Invalid [shop] value: no_value".to_string() });
                continue;
            }
        };

        let player_entity = match player_query.get_single() {
            Ok(player_entity) => player_entity,
            Err(_) => return,
        };

        shop_customer.customer = Some(player_entity);
        next_state.set(AppState::Shop);

        debug_command_info_event.send(DebugCommandInfoEvent { debug_command: "Successfully opened shop".to_string() });
    }
}


const HELP_TEXT: &str = "open shop";

pub fn push_open_shop_info(
    mut debug_command_events: EventReader<DebugCommandEvent>,
    mut debug_command_info_event: EventWriter<DebugCommandInfoEvent>,
) {
    for debug_command_event in debug_command_events.iter() {
        if debug_command_event.key != "help" {
            continue;
        }

        debug_command_info_event.send(
            DebugCommandInfoEvent { debug_command: HELP_TEXT.to_string() }
        );
    }
}