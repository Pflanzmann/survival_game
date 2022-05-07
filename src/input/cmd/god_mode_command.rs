use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, Res, With};

use crate::assets_handling::preload_player_system::PlayerConfigHandles;
use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::models::events::debug_command_info_event::DebugCommandInfoEvent;
use crate::models::player::Player;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::health::Health;

const KEY: &str = "god";

pub fn god_mode_command(
    mut commands: Commands,
    mut debug_command_events: EventReader<DebugCommandEvent>,
    mut debug_command_info_event: EventWriter<DebugCommandInfoEvent>,
    player_config: Res<PlayerConfigHandles>,
    player_query: Query<Entity, With<Player>>,
) {
    for debug_command_event in debug_command_events.iter() {
        if debug_command_event.key != KEY {
            continue;
        }

        let value = match debug_command_event.values.first() {
            Some(value) => value.to_lowercase(),
            None => continue,
        };

        let mut counter = 0;
        match value.as_str() {
            "on" => {
                for entity in player_query.iter() {
                    commands.entity(entity).remove::<Health>();
                    counter += 1;
                }

                debug_command_info_event.send(DebugCommandInfoEvent { debug_command: format!("Did remove the Health component for {} targets", counter) });
            }
            "off" => {
                for entity in player_query.iter() {
                    commands.entity(entity).insert(Health::new(player_config.player_one.health));
                    counter += 1;
                }

                debug_command_info_event.send(DebugCommandInfoEvent { debug_command: format!("Did insert a Health component for {} targets", counter) });
            }
            _ => continue,
        }
    }
}
