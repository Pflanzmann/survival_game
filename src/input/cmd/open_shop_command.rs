use bevy::prelude::{Entity, EventReader, EventWriter, Query, ResMut, With};

use crate::{AppStateTrigger, ToAppState};
use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::models::events::debug_command_info_event::DebugCommandInfoEvent;
use crate::models::player::Player;
use crate::models::resources::shop_customer::ShopCustomer;

const KEY: &str = "open_shop";

pub fn open_shop_command(
    mut debug_command_events: EventReader<DebugCommandEvent>,
    mut debug_command_info_event: EventWriter<DebugCommandInfoEvent>,
    mut shop_customer: ResMut<ShopCustomer>,
    mut state_trigger: ResMut<AppStateTrigger>,
    player_query: Query<Entity, With<Player>>,
) {
    for debug_command_event in debug_command_events.iter() {
        if debug_command_event.key != KEY {
            continue;
        }

        let player_entity = match player_query.get_single() {
            Ok(player_entity) => player_entity,
            Err(_) => return,
        };

        shop_customer.customer = Some(player_entity);
        state_trigger.state_change_trigger = ToAppState::ToShop;

        debug_command_info_event.send(DebugCommandInfoEvent { debug_command: "Successfully opened shop".to_string() });
    }
}