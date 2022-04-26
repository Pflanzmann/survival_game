use bevy::prelude::{Entity, Query, Res, Time, Transform, Vec3, With};

use crate::collision::SolidBodyCollisionQuadTreeHolder;
use crate::models::collider::collider::Collider;
use crate::models::collider::solid_body::SolidBody;
use crate::models::enemy::Enemy;
use crate::models::move_direction::MoveDirection;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_size::UnitSize;
use crate::util::is_colliding::is_colliding;
use crate::util::quad_tree::QuadData;

pub fn solid_body_collision_system(
    quad_tree_holder: Res<SolidBodyCollisionQuadTreeHolder>,
    mut solid_body_unit_query: Query<(Entity, &mut Transform, &UnitSize, &MoveDirection, &MoveSpeed, &SolidBody)>,
) {
    for (entity, mut transform, size, move_direction, move_speed, solid_body) in solid_body_unit_query.iter_mut() {
        let mut enemies_dodge_direction = Vec3::default();

        let mut check_entity_list: Vec<QuadData> = Vec::new();

        quad_tree_holder.quad_tree.query_entities(
            &mut check_entity_list,
            &transform.translation,
            &size.collider_size,
        );

        for quad_data in check_entity_list.iter() {
            if quad_data.entity == entity {
                continue;
            }

            if is_colliding(
                transform.translation,
                size.collider_size,
                quad_data.position,
                quad_data.size,
            ) {
                let vector = transform.translation - quad_data.position;

                let weight_power = if quad_data.weight > solid_body.weight {
                    1.0 - quad_data.weight
                } else {
                    1.0 - solid_body.weight
                };

                if (vector.x / (quad_data.size.x)).abs() > (vector.y / (quad_data.size.y)).abs() {
                    if vector.x < 0.0 {
                        transform.translation.x = quad_data.position.x - (quad_data.size.x);
                    } else {
                        transform.translation.x = quad_data.position.x + (quad_data.size.x);
                    }
                } else if vector.y < 0.0 {
                    transform.translation.y = quad_data.position.y - (quad_data.size.y);
                } else {
                    transform.translation.y = quad_data.position.y + (quad_data.size.y);
                }

                // let new_direction = ((quad_data.position - transform.translation).normalize_or_zero() * 2.0).round().normalize_or_zero();
                // let distance = quad_data.position.distance(transform.translation) - (quad_data.size + size.collider_size) / 2.0;
                // transform.translation += new_direction * distance.extend(0.0);
            }
        }
    }
}
