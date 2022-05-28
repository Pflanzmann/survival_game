use bevy::prelude::{Commands, Component, DespawnRecursiveExt, Entity, Query, With};

pub fn despawn_ui_system<T: Component>(
    mut commands: Commands,
    despawn_query: Query<Entity, With<T>>,
) {
    for entity in despawn_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}