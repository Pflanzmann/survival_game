use bevy::prelude::{GlobalTransform, Query, Transform, Vec3, With, Without};

use crate::models::aim_direction::AimDirection;
use crate::models::behaviour::aim_at_closest_target_behavior::AimAtClosestTargetBehavior;
use crate::models::enemy::Enemy;

pub fn aim_at_closest_target_behavior_system(
    mut behavior_query: Query<(&GlobalTransform, &mut AimDirection), (With<AimAtClosestTargetBehavior>, Without<Enemy>)>,
    target_query: Query<&Transform, With<Enemy>>,
) {
    for (behavior_transform, mut aim_direction) in behavior_query.iter_mut() {
        let mut closest_position = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut smallest_distance = f32::MAX;

        for target_transform in target_query.iter() {
            let current_distance = behavior_transform.translation.distance(target_transform.translation);

            if current_distance < smallest_distance {
                closest_position = target_transform.translation;
                smallest_distance = current_distance;
            }
        }

        aim_direction.direction = (closest_position - behavior_transform.translation).normalize_or_zero();
    }
}