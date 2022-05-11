use bevy::prelude::{Commands, DespawnRecursiveExt, EventReader};

use crate::models::events::target_died_event::TargetDiedEvent;

pub fn despawn_dead_enemy_system(
    mut commands: Commands,
    mut enemy_died_event: EventReader<TargetDiedEvent>,
) {
    for event in enemy_died_event.iter() {
        commands.entity(event.target_entity).despawn_recursive();
    }
}

