use bevy::prelude::{GlobalTransform, Query, Vec2};

use crate::models::behavior::chase_target_behavior::ChaseTargetBehavior;
use crate::models::move_direction::MoveDirection;

pub fn chase_target_behavior_system(
    mut actor_query: Query<(&GlobalTransform, &ChaseTargetBehavior, &mut MoveDirection)>,
    target_query: Query<&GlobalTransform>,
) {
    for (actor_transform, chase_target_behavior, mut move_direction) in actor_query.iter_mut() {
        let target_transform = match target_query.get(chase_target_behavior.target) {
            Ok(value) => value,
            Err(_) => continue
        };

        if actor_transform.translation().truncate().distance(target_transform.translation().truncate()) < chase_target_behavior.proximity {
            move_direction.direction = Vec2::default();
        } else {
            move_direction.direction = (target_transform.translation().truncate() - actor_transform.translation().truncate()).normalize_or_zero();
        }
    }
}