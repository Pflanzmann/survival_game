use bevy::prelude::{Entity, Query, Res, Transform, Vec2};

use crate::models::collision::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::collision::collider_weight::ColliderWeight;
use crate::models::collision::solid_body_collider::SolidBodyCollider;
use crate::models::resources::collision::solid_body_quad_tree::{SolidBodyData, SolidBodyQuadTree};
use crate::util::quad_tree::QuadData;

pub fn solid_body_collision_system(
    quad_tree_holder: Res<SolidBodyQuadTree>,
    mut solid_body_unit_query: Query<(Entity, &mut Transform, &SolidBodyCollider, &ColliderWeight)>,
) {
    solid_body_unit_query.par_for_each_mut(10, |(entity, mut transform, solid_body_collider, collision_weight)| {
        let size = match solid_body_collider.collider_type {
            Circle(radius) => Vec2::new(radius, radius),
            Rectangle(size) => size,
        };

        let self_position = transform.translation.truncate() + solid_body_collider.offset;

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

            if solid_body_collider.collider_type.is_colliding(&self_position, &quad_data.data.collider_type, &quad_data.position) {
                let resolution_position = solid_body_collider.collider_type.get_collision_resolution_position(
                    &self_position,
                    collision_weight.weight,
                    &quad_data.data.collider_type,
                    &quad_data.position,
                    quad_data.data.collision_weight,
                );

                collision_resolutions += resolution_position;
                collision_resolutions_counter += 1.0;
            }
        }

        if collision_resolutions.length() > 0.0 {
            transform.translation = ((collision_resolutions / collision_resolutions_counter) - solid_body_collider.offset).extend(transform.translation.z);
        }
    });
}
