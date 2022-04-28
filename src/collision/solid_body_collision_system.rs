use bevy::prelude::{Entity, Query, Res, Time, Transform, Vec2, Vec3, With};

use crate::collision::SolidBodyCollisionQuadTreeHolder;
use crate::models::collider::collider_type::ColliderType;
use crate::models::collider::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::collider::solid_body::SolidBody;
use crate::models::move_direction::MoveDirection;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::util::quad_tree::QuadData;

pub fn solid_body_collision_system(
    time: Res<Time>,
    quad_tree_holder: Res<SolidBodyCollisionQuadTreeHolder>,
    mut solid_body_unit_query: Query<(Entity, &mut Transform, &ColliderType, &MoveDirection, &MoveSpeed, &SolidBody)>,
) {
    for (entity, mut transform, unit_size, move_direction, move_speed, solid_body) in solid_body_unit_query.iter_mut() {
        let size = match unit_size {
            Circle(radius) => Vec2::new(*radius, *radius),
            Rectangle(size) => *size,
        };

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

            if quad_data.collider_type.is_colliding(&quad_data.position.truncate(), unit_size, &transform.translation.truncate()) {
                let new_direction = -(quad_data.position - transform.translation).normalize_or_zero();
                let distance = 256.0;
                transform.translation += -move_direction.direction * (move_speed.get_total_amount() * time.delta_seconds() * 60.0);


                // let vector = transform.translation - quad_data.position;

                // if (vector.x / (quad_data.size.x)).abs() > (vector.y / (quad_data.size.y)).abs() {
                //     if vector.x < 0.0 {
                //         transform.translation.x = quad_data.position.x - (quad_data.size.x);
                //     } else {
                //         transform.translation.x = quad_data.position.x + (quad_data.size.x);
                //     }
                // } else if vector.y < 0.0 {
                //     transform.translation.y = quad_data.position.y - (quad_data.size.y);
                // } else {
                //     transform.translation.y = quad_data.position.y + (quad_data.size.y);
                // }
            }
        }
    }
}
