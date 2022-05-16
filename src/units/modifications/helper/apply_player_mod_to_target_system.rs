use bevy::prelude::{Commands, Component, EventReader, Query, With};

use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::modifications::descriptors::modification::Modification;

/// A generic system to apply a [Bullet][Modification] from the source
/// to the target of the [ApplyModToTargetSystem].
///
/// The mod gets applied from the source [Entity] with a [Modification]-Tag.
///
/// ```
/// # use bevy_ecs::prelude::;
/// #
/// impl Plugin for ExamplePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_system(apply_player_mod_to_target_system::<CurveShot>)
///     }
/// }
/// ```
pub fn apply_player_mod_to_target_system<T: Component + Clone>(
    mut commands: Commands,
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mod_query: Query<&T, With<Modification>>,
) {
    for apply_event in apply_events.iter() {
        let player_mod = match mod_query.get(apply_event.mod_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        commands.entity(apply_event.target_entity).insert(player_mod.clone());
    }
}