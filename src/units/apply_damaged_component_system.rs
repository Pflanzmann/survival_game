use bevy::prelude::{Commands, EventReader};

use crate::models::damaged_effect::DamagedEffect;
use crate::models::events::damaged_event::DamagedEvent;

pub fn apply_damage_component_system(
    mut commands: Commands,
    mut enemy_damaged_event: EventReader<DamagedEvent>,
) {
    for event in enemy_damaged_event.iter() {
        commands.entity(event.damaged_entity).insert(DamagedEffect::new());
    }
}
