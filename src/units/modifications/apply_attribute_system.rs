use bevy::prelude::{Added, Component, Entity, Query};

use crate::models::attributes::attribute::{Attribute, AttributeAffect};
use crate::models::modification_components::ModContainerSlot;

pub fn apply_attribute_system<
    T: Component + Attribute,
    U: Component + AttributeAffect<T>>(
    mod_container_query: Query<(Entity, &U), Added<U>>,
    mut container_owner_query: Query<(&ModContainerSlot, &mut T)>,
) {
    for (container_entity, affect) in mod_container_query.iter() {
        for (owner_slot, mut attribute) in container_owner_query.iter_mut() {
            if owner_slot.container_entity == container_entity {
                affect.add_affect(&mut attribute)
            }
        }
    }
}