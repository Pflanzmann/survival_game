use bevy::prelude::{Component, EventReader, Query, With};

use crate::models::attribute_container::AttributeContainer;
use crate::models::attribute_container_slot::AttributeContainerSlot;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::modifications::affects::attribute_affect::AttributeAffect;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::weapon_slot::WeaponSlot;

/// A generic system to apply an [AttributeAffect] like [AffectMoveSpeed] from the source
/// to the target of the [ApplyModToTargetSystem].
///
/// ```
/// # use bevy_ecs::prelude::;
/// #
/// impl Plugin for ExamplePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_system(apply_projectile_affect_system::<MoveSpeed, AffectProjectileMoveSpeed>)
///     }
/// }
/// ```
pub fn apply_projectile_affect_system<
    T: Component + Attribute,
    U: Component + AttributeAffect<T>>
(
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    affect_query: Query<&U>,
    gun_holder_query: Query<&WeaponSlot>,
    gun_query: Query<&AttributeContainerSlot>,
    mut target_query: Query<&mut T, With<AttributeContainer>>,
) {
    for apply_event in apply_events.iter() {
        let affect = match affect_query.get(apply_event.mod_entity) {
            Ok(affect) => affect,
            Err(_) => continue,
        };

        let weapon_slow = match gun_holder_query.get(apply_event.target_entity) {
            Ok(attribute) => attribute,
            Err(_) => continue,
        };

        let attribute_container_slot = match gun_query.get(weapon_slow.weapon_entity) {
            Ok(attribute) => attribute,
            Err(_) => continue,
        };

        let mut target = match target_query.get_mut(attribute_container_slot.container_entity) {
            Ok(affect) => affect,
            Err(_) => continue,
        };

        affect.add_affect(&mut target);
    }
}
