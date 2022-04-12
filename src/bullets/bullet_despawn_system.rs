use bevy::prelude::{Commands, EventReader};

use crate::models::events::bullet_stopped_event::BulletStoppedEvent;

/// This system handles the despawn of [`Bullet`] as a reaction to the [`BulletStoppedEvent`].
pub fn bullet_despawn_system(
    mut commands: Commands,
    mut bullet_stopped_event: EventReader<BulletStoppedEvent>,
) {
    for event in bullet_stopped_event.iter() {
        commands.entity(event.bullet_entity).despawn();
    }
}