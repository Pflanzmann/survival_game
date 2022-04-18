use bevy::prelude::{Commands, Entity, EventReader, Query, Res, With};
use crate::assets_handling::preload_player_system::PlayerConfigHandles;

use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::models::player::Player;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::health::Health;

const KEY: &str = "god";

pub fn god_mode_command(
    mut commands: Commands,
    mut debug_command_events: EventReader<DebugCommandEvent>,
    player_config: Res<PlayerConfigHandles>,
    player_query: Query<Entity, With<Player>>,
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

        let parameter = match arguments.next() {
            Some(value) => value.to_lowercase(),
            None => continue,
        };

        match parameter.to_lowercase().as_str() {
            "on" => {
                for entity in player_query.iter() {
                    commands.entity(entity).remove::<Health>();
                }
            }
            "off" => {
                for entity in player_query.iter() {
                    commands.entity(entity).insert(Health::new(player_config.player_one.health));
                }
            }
            _ => continue,
        }
    }
}
