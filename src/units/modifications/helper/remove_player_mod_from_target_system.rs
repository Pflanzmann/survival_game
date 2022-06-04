use bevy::prelude::{Commands, Component, EventReader, Query};

use crate::models::events::remove_mod_from_target_event::RemoveModFromTargetEvent;

/// A generic system to remove a [Projectile][Modification] from the target.
///
/// ```
/// # use bevy_ecs::prelude::;
/// #
/// impl Plugin for ExamplePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_system(remove_player_mod_from_target_system::<CurveShot>)
///     }
/// }
/// ```
pub fn remove_player_mod_from_target_system<T: Component>(
    mut commands: Commands,
    mut remove_events: EventReader<RemoveModFromTargetEvent>,
    target_query: Query<&T>,
) {
    for remove_event in remove_events.iter() {
        if target_query.get(remove_event.target_entity).is_ok() {
            commands.entity(remove_event.target_entity).remove::<T>();
        }
    }
}