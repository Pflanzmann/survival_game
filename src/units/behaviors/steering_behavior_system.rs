use bevy::prelude::*;

use crate::collision::SolidBodyCollisionQuadTreeHolder;
use crate::models::behavior::steering_behavior::SteeringBehavior;
use crate::models::collider::collider_type::ColliderType;
use crate::models::collider::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::move_direction::MoveDirection;
use crate::util::quad_tree::QuadData;

pub fn steering_behavior_system(
    mut units_query: Query<(Entity, &Transform, &ColliderType, &MoveDirection, &mut SteeringBehavior)>,
    quad_tree_holder: Res<SolidBodyCollisionQuadTreeHolder>,
) {
    for (entity, transform, collider_type, move_direction, mut steering_behavior) in units_query.iter_mut() {
        let size = match collider_type {
            Circle(radius) => Vec2::new(*radius, *radius),
            Rectangle(size) => *size,
        };

        let mut check_entity_list: Vec<QuadData> = Vec::new();

        let check_position = transform.translation + (move_direction.direction * (size.x * 1.0));

        quad_tree_holder.quad_tree.query_entities(
            &mut check_entity_list,
            &check_position,
            &(size * 1.5),
        );

        let mut closest_thread: Option<QuadData> = Option::None;
        let mut closest_distance: f32 = 10000000000000000000.0;
        for quad_data in check_entity_list.iter() {
            if quad_data.entity == entity {
                continue;
            }

            if collider_type.is_colliding(&check_position.truncate(), &quad_data.collider_type, &quad_data.position.truncate()) {
                let distance = transform.translation.truncate().distance(quad_data.position.truncate());

                if distance < closest_distance {
                    closest_distance = distance;
                    closest_thread = Some(*quad_data);
                }
            }
        }

        if let Some(thread) = closest_thread {
            let new_check_position = transform.translation + (move_direction.direction * transform.translation.distance(thread.position));
            let avoid_direction = (new_check_position - thread.position).normalize_or_zero();

            let avoid_position = check_position + (avoid_direction * 500.0);

            steering_behavior.direction = (move_direction.direction + (avoid_position - transform.translation)).normalize_or_zero().truncate();
        } else {
            steering_behavior.direction = Vec2::default();
        }
    }
}