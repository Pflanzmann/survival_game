use bevy::prelude::{Commands, Component, EventReader, Query, With, Without};

use crate::models::events::remove_mod_from_target_event::RemoveModFromTargetEvent;
use crate::models::mod_container_slot::ModContainerSlot;
use crate::models::weapon_slot::WeaponSlot;

/// A generic system to remove a [Bullet][Modification] from a source
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
///         app.add_system(remove_bullet_mod_from_targets_gun_system::<CurveShot>)
///     }
/// }
/// ```
pub fn remove_bullet_mod_from_targets_gun_system<T: Component>(
    mut commands: Commands,
    mut remove_events: EventReader<RemoveModFromTargetEvent>,
    holder_query: Query<&WeaponSlot, Without<ModContainerSlot>>,
    gun_query: Query<&ModContainerSlot, Without<WeaponSlot>>,
) {
    for remove_event in remove_events.iter() {
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