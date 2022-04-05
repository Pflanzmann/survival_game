use bevy::prelude::{Commands, EventReader};

use crate::entities::events::bullet_stopped_event::BulletStoppedEvent;

pub fn bullet_despawn_system(
    mut commands: Commands,
    mut bullet_stopped_event: EventReader<BulletStoppedEvent>,
) {
    for event in bullet_stopped_event.iter() {
        commands.entity(event.bullet_entity).despawn();
    }
}