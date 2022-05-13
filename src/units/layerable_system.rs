use bevy::prelude::{Changed, Query, Transform};

use crate::models::layerable::Layerable;

pub fn layerable_system(
    mut layerable_query: Query<(&mut Transform, &Layerable), Changed<Transform>>
) {
    for (mut transform, layerable) in layerable_query.iter_mut() {
        transform.translation.z = layerable.layer - transform.translation.y;
    }
}
