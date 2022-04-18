use bevy::math::{EulerRot, Quat};
use bevy::prelude::{Query, Transform, With};

use crate::models::behavior::rotate_behavior::UnitRotation;

pub fn rotate_unit_system(
    mut unit_query: Query<(&mut Transform, &UnitRotation)>,
) {
    for (mut unit_transform, unit_rotation) in unit_query.iter_mut() {
        unit_transform.rotate(Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, unit_rotation.angle)
        )
    }
}