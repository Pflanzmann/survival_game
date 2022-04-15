use bevy::prelude::{Commands, EventReader};

use crate::models::events::enemy_died_event::EnemyDiedEvent;

pub fn despawn_dead_enemy_system(
    mut commands: Commands,
    mut enemy_died_event: EventReader<EnemyDiedEvent>,
) {
    for event in enemy_died_event.iter() {
        commands.entity(event.enemy_entity).despawn();
    }
}

