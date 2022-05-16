use bevy::prelude::{Component, EventReader, Query};

use crate::models::events::remove_mod_from_target_event::RemoveModFromTargetEvent;
use crate::models::modifications::affects::attribute_affect::AttributeAffect;
use crate::models::unit_attributes::attribute::Attribute;

/// A generic system to remove [AttributeAffect]s like [AffectMoveSpeed] from the source
/// to the target of the [ApplyModToTargetSystem].
///
/// The affect changes get removed from the target [Entity].
///
/// ```
/// # use bevy_ecs::prelude::;
/// #
/// impl Plugin for ExamplePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_system(remove_affect_system::<MoveSpeed, AffectMoveSpeed>)
///     }
/// }
/// ```
pub fn remove_affect_system<
    T: Component + Attribute,
    U: Component + AttributeAffect<T>>(
    mut apply_events: EventReader<RemoveModFromTargetEvent>,
    mut target_query: Query<&mut T>,
    affect_query: Query<&U>,
) {
    for remove_event in apply_events.iter() {
        let affect = match affect_query.get(remove_event.mod_entity) {
            Ok(affect) => affect,
            Err(_) => continue,
        };

        let mut target_attribute = match target_query.get_mut(remove_event.target_entity) {
            Ok(attribute) => attribute,
            Err(_) => continue,
        };

        affect.remove_affect(&mut target_attribute);
    }
}
