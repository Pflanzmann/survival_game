use bevy::prelude::*;

use crate::models::behavior::steering_behavior::SteeringBehavior;
use crate::models::collision::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::collision::solid_body_collider::SolidBodyCollider;
use crate::models::move_direction::MoveDirection;
use crate::models::resources::solid_body_quad_tree::{SolidBodyData, SolidBodyQuadTree};
use crate::util::quad_tree::QuadData;

pub fn steering_behavior_system(
    mut units_query: Query<(Entity, &Transform, &SolidBodyCollider, &MoveDirection, &mut SteeringBehavior)>,
    quad_tree_holder: Res<SolidBodyQuadTree>,
) {
    for (entity, transform, solid_body_collider, move_direction, mut steering_behavior) in units_query.iter_mut() {
        let size = match solid_body_collider.collider_type {
            Circle(radius) => Vec2::new(radius, radius),
            Rectangle(size) => size,
        };

        let mut check_entity_list: Vec<QuadData<SolidBodyData>> = Vec::new();

        let unit_position = transform.translation + solid_body_collider.offset;
        let check_position = unit_position + (move_direction.direction * (size.x * 0.5));

        quad_tree_holder.query_entities(
            &mut check_entity_list,
            &check_position,
            &(size * 2.0),
        );

        let mut closest_thread: Option<QuadData<SolidBodyData>> = Option::None;
        let mut closest_distance: f32 = 10000.0;
        for quad_data in check_entity_list.iter() {
            if quad_data.data.entity == entity {
                continue;
            }

            if solid_body_collider.collider_type.is_colliding(&check_position.truncate(), &quad_data.data.collider_type, &quad_data.position.truncate()) {
                let distance = unit_position.truncate().distance(quad_data.position.truncate());

                if distance < closest_distance {
                    closest_distance = distance;
                    closest_thread = Some(*quad_data);
                }
            }
        }

        if let Some(thread) = closest_thread {
            let new_check_position = (unit_position) + (move_direction.direction * (unit_position).distance(thread.position));
            let avoid_direction = (new_check_position - thread.position).normalize_or_zero();

            let avoid_position = check_position + (avoid_direction * 300.0);

            steering_behavior.direction = (move_direction.direction + (avoid_position - (unit_position))).normalize_or_zero().truncate();
        } else {
            steering_behavior.direction = Vec2::default();
        }
    }
}