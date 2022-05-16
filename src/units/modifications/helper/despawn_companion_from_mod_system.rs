use bevy::prelude::{Commands, Component, DespawnRecursiveExt, Entity, EventReader, Query, With};

use crate::models::events::remove_mod_from_target_event::RemoveModFromTargetEvent;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::modifications::utils::owner::Owner;

/// A generic system to remove all companions created from a [Modification].
///
/// All associated companions get despawned recursively when the RemoveModFromTargetEvent
/// gets called.
///
/// ```
/// # use bevy_ecs::prelude::*;
/// #
/// impl Plugin for ExamplePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_system(despawn_companion_from_mod_system::<Turret, TurretUnit>)
///     }
/// }
/// ```
pub fn despawn_companion_from_mod_system<
    T: Component,
    U: Component
>(
    mut commands: Commands,
    mut remove_events: EventReader<RemoveModFromTargetEvent>,
    mod_query: Query<Entity, With<Modification>>,
    unit_query: Query<(Entity, &Owner), With<U>>,
) {
    for remove_event in remove_events.iter() {
        if mod_query.get(remove_event.mod_entity).is_err() {
            continue;
        };

        for (unit_entity, unit_owner) in unit_query.iter() {
            if unit_owner.entity == remove_event.target_entity {
                commands.entity(unit_entity).despawn_recursive();
            }
        }
    }
}