use bevy::core::Time;
use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Query, Res};

use crate::models::time_alive::TimeAlive;

pub fn time_alive_system(
    mut commands: Commands,
    time: Res<Time>,
    mut entity_query: Query<(Entity, &mut TimeAlive)>,
) {
    for (entity, mut time_alive) in entity_query.iter_mut() {
        time_alive.time_alive -= time.delta_seconds();

        if time_alive.time_alive <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
