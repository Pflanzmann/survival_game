use bevy::prelude::{Changed, GlobalTransform, Query, Transform};

use crate::models::collision::solid_body_collider::SolidBodyCollider;
use crate::models::layerable::Layerable;

pub fn layerable_system(
    mut layerable_query: Query<(&mut Transform, &Layerable, Option<&SolidBodyCollider>), Changed<Transform>>
) {
    for (mut transform, layerable, solid_body_collider) in layerable_query.iter_mut() {
        let offset = match solid_body_collider {
            Some(solid_body_collider) => solid_body_collider.offset.y,
            None => 0.0,
        };

        transform.translation.z = layerable.layer - transform.translation.y + offset;
    }
}
