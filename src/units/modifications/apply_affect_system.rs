use bevy::prelude::{Component, Query};

use crate::EventReader;
use crate::models::attributes::attribute::Attribute;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetSystem;
use crate::models::modification_attributes::attribute_affect::AttributeAffect;

pub fn apply_attribute_system<
    T: Component + Attribute,
    U: Component + AttributeAffect<T>>(
    mut apply_events: EventReader<ApplyModToTargetSystem>,
    mut target_query: Query<&mut T>,
    mod_query: Query<&U>,
) {
    for apply_event in apply_events.iter() {
        let mut target_attribute = match target_query.get_mut(apply_event.target_entity) {
            Ok(attribute) => attribute,
            Err(_) => continue,
        };

        let modification = match mod_query.get(apply_event.mod_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        modification.add_affect(&mut target_attribute);
    }
}
