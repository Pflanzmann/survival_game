use bevy::prelude::{EventReader, EventWriter, ResMut};

use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::models::events::debug_command_info_event::DebugCommandInfoEvent;
use crate::models::resources::world::spawn_phase_timer::SpawnStageState;

const KEY: &str = "spawn";

pub fn toggle_spawner_command(
    mut debug_command_events: EventReader<DebugCommandEvent>,
    mut debug_command_info_event: EventWriter<DebugCommandInfoEvent>,
    mut spawn_stage_state: ResMut<SpawnStageState>,
) {
    for debug_command_event in debug_command_events.iter() {
        if debug_command_event.key != KEY {
            continue;
        }

        let value = match debug_command_event.values.first() {
            Some(value) => value,
            None => {
                debug_command_info_event.send(DebugCommandInfoEvent { debug_command: "Invalid [spawn] value: no_value".to_string() });
                continue;
            }
        };

        match value.as_str() {
            "on" => {
                spawn_stage_state.phase_timer = 0.0;
                spawn_stage_state.spawn_interval = 0.0;
                spawn_stage_state.current_spawn_phase -= 1;
                debug_command_info_event.send(DebugCommandInfoEvent { debug_command: "Enable spawning".to_string() });
            }

            "off" => {
                spawn_stage_state.phase_timer = 1000000000.0;
                spawn_stage_state.spawn_interval = 1000000000.0;
                debug_command_info_event.send(DebugCommandInfoEvent { debug_command: "Disable spawning".to_string() });
            }

            _ => {
                debug_command_info_event.send(DebugCommandInfoEvent { debug_command: format!("Invalid [spawn] value: {}", value) });
                continue;
            }
        }
    }
}


const HELP_TEXT: &str = "spawn [toggle (on | off)]";

pub fn push_toggle_spawner_info(
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