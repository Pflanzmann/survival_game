use bevy::prelude::{Component, Query};

use crate::EventReader;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetSystem;
use crate::models::modifications::affects::attribute_affect::AttributeAffect;

pub fn apply_affect_system<
    T: Component + Attribute,
    U: Component + AttributeAffect<T>>(
    mut apply_events: EventReader<ApplyModToTargetSystem>,
    mut target_query: Query<&mut T>,
    affect_query: Query<&U>,
) {
    for apply_event in apply_events.iter() {
        let mut target_attribute = match target_query.get_mut(apply_event.target_entity) {
            Ok(attribute) => attribute,
            Err(_) => continue,
        };

        let affect = match affect_query.get(apply_event.mod_entity) {
            Ok(affect) => affect,
            Err(_) => continue,
        };

        affect.add_affect(&mut target_attribute);
    }
}
