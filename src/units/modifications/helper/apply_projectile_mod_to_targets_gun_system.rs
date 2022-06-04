use bevy::prelude::{Commands, Component, EventReader, Query, With, Without};

use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::mod_container_slot::ModContainerSlot;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::player::Player;
use crate::models::weapon_slot::WeaponSlot;

/// A generic system to apply a [Projectile][Modification] from the source
/// to the target of the [ApplyModToTargetSystem].
///
/// The modification gets applied (cloned) from the source [Entity] with a [Modification]-Tag to
/// the [ModContainer] from the [WeaponSlot] of the target [Entity].
///
/// ```
/// # use bevy_ecs::prelude::;
/// #
/// impl Plugin for ExamplePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_system(apply_projectile_mod_to_targets_gun_system::<CurveShot>)
///     }
/// }
/// ```
pub fn apply_projectile_mod_to_targets_gun_system<T: Component + Clone>(
    mut commands: Commands,
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mod_query: Query<&T, With<Modification>>,
    holder_query: Query<&WeaponSlot, With<Player>>,
    gun_query: Query<&ModContainerSlot, Without<Player>>,
) {
    for apply_event in apply_events.iter() {
        let projectile_mod = match mod_query.get(apply_event.mod_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        let holder_weapon_slot = match holder_query.get(apply_event.target_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        let weapon_mod_container = match gun_query.get(holder_weapon_slot.weapon_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        commands.entity(weapon_mod_container.container_entity).insert(projectile_mod.clone());
    }
}