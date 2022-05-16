use bevy::prelude::{Component, EventReader, Query};

use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::modifications::affects::attribute_affect::AttributeAffect;
use crate::models::unit_attributes::attribute::Attribute;

/// A generic system to apply an [AttributeAffect] like [AffectMoveSpeed] from the source
/// to the target of the [ApplyModToTargetSystem].
//////
/// ```
/// # use bevy_ecs::prelude::;
/// #
/// impl Plugin for ExamplePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_system(apply_affect_system::<MoveSpeed, AffectMoveSpeed>)
///     }
/// }
/// ```
pub fn apply_affect_system<
    T: Component + Attribute,
    U: Component + AttributeAffect<T>>(
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mut target_query: Query<&mut T>,
    affect_query: Query<&U>,
) {
    for apply_event in apply_events.iter() {
        let affect = match affect_query.get(apply_event.mod_entity) {
            Ok(affect) => affect,
            Err(_) => continue,
        };

        let mut target_attribute = match target_query.get_mut(apply_event.target_entity) {
            Ok(attribute) => attribute,
            Err(_) => continue,
        };

        affect.add_affect(&mut target_attribute);
    }
}
