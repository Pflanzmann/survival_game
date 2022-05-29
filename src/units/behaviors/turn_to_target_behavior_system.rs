use bevy::prelude::{GlobalTransform, Query, Res, Time, Transform, Vec2};

use crate::models::aim_direction::AimDirection;
use crate::models::behavior::turn_to_target_behavior::TurnToTargetBehavior;

pub fn turn_to_target_behavior_system(
    time: Res<Time>,
    mut units_query: Query<(&Transform, &mut AimDirection, &TurnToTargetBehavior)>,
    target_query: Query<&GlobalTransform>,
) {
    for (actor_transform, mut aim_direction, turn_to_target_behavior) in units_query.iter_mut() {
        let target_position = match target_query.get(turn_to_target_behavior.target) {
            Ok(value) => value.translation.truncate(),
            Err(_) => continue
        };

        let direction = (target_position - actor_transform.translation.truncate()).normalize_or_zero();
        let aim_angle = aim_direction.direction.y.atan2(aim_direction.direction.x);
        let angle_between = aim_direction.direction.angle_between(direction);

        if angle_between.abs() < 0.1 {
            continue;
        }

        let angle = if angle_between > 0.0 {
            aim_angle + 60.0 / 360.0 * turn_to_target_behavior.revolutions_per_min * time.delta_seconds()
        } else {
            aim_angle - 60.0 / 360.0 * turn_to_target_behavior.revolutions_per_min * time.delta_seconds()
        };

        let new_aim_direction = Vec2::new(
            angle.cos(),
            angle.sin(),
        ).normalize_or_zero();

        aim_direction.direction = new_aim_direction;
    }
}
