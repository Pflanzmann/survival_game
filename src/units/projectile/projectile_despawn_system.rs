use bevy::prelude::{Commands, DespawnRecursiveExt, EventReader};

use crate::models::events::projectile_stopped_event::ProjectileStoppedEvent;

/// This system handles the despawn of [Projectile] as a reaction to the [ProjectileStoppedEvent].
pub fn projectile_despawn_system(
    mut commands: Commands,
    mut projectile_stopped_event: EventReader<ProjectileStoppedEvent>,
) {
    for event in projectile_stopped_event.iter() {
        commands.entity(event.projectile_entity).despawn_recursive();
    }
}