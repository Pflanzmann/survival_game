use bevy::prelude::{Changed, Parent, Query, With, Without};

use crate::models::inherit_unit_size::InheritUnitSize;
use crate::models::unit_attributes::unit_size::UnitSize;

pub fn inherit_unit_size_system(
    parent_query: Query<&UnitSize, (Changed<UnitSize>, Without<Parent>)>,
    mut child_query: Query<(&mut UnitSize, &Parent), (With<InheritUnitSize>, With<Parent>)>,
) {
    for (mut child_unit_size, parent) in child_query.iter_mut() {
        if let Ok(parent_unit_size) = parent_query.get(parent.get()) {
            child_unit_size.inherit_from(parent_unit_size);
        }
    }
}