use bevy::math::{EulerRot, Quat};
use bevy::prelude::{Query, Res, Time, Transform};

use crate::models::behavior::rotate_behavior::UnitRotation;

pub fn rotate_unit_system(
    time: Res<Time>,
    mut unit_query: Query<(&mut Transform, &UnitRotation)>,
) {
    for (mut unit_transform, unit_rotation) in unit_query.iter_mut() {
        unit_transform.rotate(Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 60.0 / 360.0 * unit_rotation.revolutions_per_min * time.delta_seconds()))
    }
}