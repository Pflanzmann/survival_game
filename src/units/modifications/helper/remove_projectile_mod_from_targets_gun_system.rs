use bevy::prelude::{Commands, Component, EventReader, Query, With};

use crate::models::events::remove_mod_from_target_event::RemoveModFromTargetEvent;
use crate::models::mod_container_slot::ModContainerSlot;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::weapon_slot::WeaponSlot;

/// A generic system to remove a [Projectile][Modification] from a source
/// to the target´s gun of the [ApplyModToTargetSystem].
///
/// The modification gets removed from the [ModContainer] of the gun that gets held by
/// the target [Entity].
///
/// ```
/// # use bevy_ecs::prelude::;
/// #
/// impl Plugin for ExamplePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_system(remove_projectile_mod_from_targets_gun_system::<CurveShot>)
///     }
/// }
/// ```
pub fn remove_projectile_mod_from_targets_gun_system<T: Component>(
    mut commands: Commands,
    mut remove_events: EventReader<RemoveModFromTargetEvent>,
    mod_query: Query<&T, With<Modification>>,
    holder_query: Query<&WeaponSlot>,
    gun_query: Query<&ModContainerSlot>,
) {
    for remove_event in remove_events.iter() {
        if mod_query.get(remove_event.mod_entity).is_err() {
            continue;
        }

        let holder_weapon_slot = match holder_query.get(remove_event.target_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        let weapon_mod_container = match gun_query.get(holder_weapon_slot.weapon_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        commands.entity(weapon_mod_container.container_entity).remove::<T>();
    }
}