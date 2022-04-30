use bevy::prelude::{Entity, Query, Res, Transform, Vec2};

use crate::collision::SolidBodyCollisionQuadTreeHolder;
use crate::models::collider::collider_type::ColliderType;
use crate::models::collider::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::collider::solid_body::ColliderWeight;
use crate::util::quad_tree::QuadData;

pub fn solid_body_collision_system(
    quad_tree_holder: Res<SolidBodyCollisionQuadTreeHolder>,
    mut solid_body_unit_query: Query<(Entity, &mut Transform, &ColliderType, &ColliderWeight)>,
) {
    for (entity, mut transform, collider_type, collider_weight) in solid_body_unit_query.iter_mut() {
        let size = match collider_type {
            Circle(radius) => Vec2::new(*radius, *radius),
            Rectangle(size) => *size,
        };

        let mut collision_resolutions: Vec2 = Vec2::default();
        let mut collision_resolutions_counter = 0.0;

        let mut check_entity_list: Vec<QuadData> = Vec::new();
        quad_tree_holder.quad_tree.query_entities(
            &mut check_entity_list,
            &transform.translation,
            &size,
        );

        for quad_data in check_entity_list.iter() {
            if quad_data.entity == entity {
                continue;
            }

            if collider_type.is_colliding(&transform.translation.truncate(), &quad_data.collider_type, &quad_data.position.truncate()) {
                let resolution_position = collider_type.get_collision_resolution_position(&transform.translation.truncate(), &quad_data.collider_type, &quad_data.position.truncate());

                collision_resolutions += resolution_position;
                collision_resolutions_counter += 1.0;
            }
        }

        if collision_resolutions.length() > 0.0 {
            transform.translation = (collision_resolutions / collision_resolutions_counter).extend(transform.translation.z);
        }
    }
}
