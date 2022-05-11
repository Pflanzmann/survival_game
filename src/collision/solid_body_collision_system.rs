use bevy::prelude::{Entity, Query, Res, Transform, Vec2};

use crate::models::collision::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::collision::collider_weight::ColliderWeight;
use crate::models::collision::solid_body_collider::SolidBodyCollider;
use crate::models::resources::solid_body_quad_tree::{SolidBodyData, SolidBodyQuadTree};
use crate::util::quad_tree::QuadData;

pub fn solid_body_collision_system(
    quad_tree_holder: Res<SolidBodyQuadTree>,
    mut solid_body_unit_query: Query<(Entity, &mut Transform, &SolidBodyCollider, &ColliderWeight)>,
) {
    for (entity, mut transform, solid_body_collider, collision_weight) in solid_body_unit_query.iter_mut() {
        let size = match solid_body_collider.collider_type {
            Circle(radius) => Vec2::new(radius, radius),
            Rectangle(size) => size,
        };

        let self_position = transform.translation + solid_body_collider.offset;

        let mut collision_resolutions: Vec2 = Vec2::default();
        let mut collision_resolutions_counter = 0.0;

        let mut check_entity_list: Vec<QuadData<SolidBodyData>> = Vec::new();
        quad_tree_holder.query_entities(
            &mut check_entity_list,
            &self_position,
            &size,
        );

        for quad_data in check_entity_list.iter() {
            if quad_data.data.entity == entity {
                continue;
            }

            if solid_body_collider.collider_type.is_colliding(&self_position.truncate(), &quad_data.data.collider_type, &quad_data.position.truncate()) {
                let resolution_position = solid_body_collider.collider_type.get_collision_resolution_position(
                    &self_position.truncate(),
                    collision_weight.weight,
                    &quad_data.data.collider_type,
                    &quad_data.position.truncate(),
                    quad_data.data.collision_weight,
                );

                collision_resolutions += resolution_position;
                collision_resolutions_counter += 1.0;
            }
        }

        if collision_resolutions.length() > 0.0 {
            transform.translation = (collision_resolutions / collision_resolutions_counter).extend(transform.translation.z) - solid_body_collider.offset;
        }
    }
}
