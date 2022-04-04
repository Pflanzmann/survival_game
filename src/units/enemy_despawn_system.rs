use bevy::prelude::{Commands, EventReader};

use crate::components::event_components::EnemyDiedEvent;

pub fn enemy_despawn_system(
    mut commands: Commands,
    mut enemy_died_event: EventReader<EnemyDiedEvent>,
) {
    for event in enemy_died_event.iter() {
        commands.entity(event.enemy_entity).despawn();
    }
}