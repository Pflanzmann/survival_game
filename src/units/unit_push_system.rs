use bevy::prelude::{Commands, Entity, Query, Res, Time, Transform};

use crate::models::collision::collider_weight::ColliderWeight;
use crate::models::unit_push::UnitPush;

pub fn unit_push_system(
    mut commands: Commands,
    time: Res<Time>,
    mut push_query: Query<(Entity, &mut Transform, &mut UnitPush, &ColliderWeight)>,
) {
    for (entity, mut transform, mut unit_push, collider_weight) in push_query.iter_mut() {
        transform.translation += unit_push.direction.extend(0.0) * unit_push.force * time.delta_seconds() * (1.0 - collider_weight.weight);

        unit_push.duration -= time.delta_seconds();
        if unit_push.duration <= 0.0 {
            commands.entity(entity).remove::<UnitPush>();
        }
    }
}