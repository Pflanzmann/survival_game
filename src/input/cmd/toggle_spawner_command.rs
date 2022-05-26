use bevy::prelude::{EventReader, EventWriter, ResMut};

use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::models::events::debug_command_info_event::DebugCommandInfoEvent;
use crate::models::resources::spawn_phase_timer::SpawnPhaseTimer;
use crate::models::resources::spawn_timer::SpawnIntervalTimer;

const KEY: &str = "spawn";

pub fn toggle_spawner_command(
    mut debug_command_events: EventReader<DebugCommandEvent>,
    mut debug_command_info_event: EventWriter<DebugCommandInfoEvent>,
    mut spawn_phase_timer: ResMut<SpawnPhaseTimer>,
    mut spawn_interval_timer: ResMut<SpawnIntervalTimer>,
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
                spawn_phase_timer.timer = 0.0;
                spawn_interval_timer.timer = 0.0;
                spawn_phase_timer.current_spawn_phase -= 1;
                debug_command_info_event.send(DebugCommandInfoEvent { debug_command: "Enable spawning".to_string() });
            }

            "off" => {
                spawn_phase_timer.timer = 1000000000.0;
                spawn_interval_timer.timer = 1000000000.0;
                debug_command_info_event.send(DebugCommandInfoEvent { debug_command: "Disable spawning".to_string() });
            }

            _ => {
                debug_command_info_event.send(DebugCommandInfoEvent { debug_command: format!("Invalid [spawn] value: {}", value) });
                continue;
            }
        }
    }
}