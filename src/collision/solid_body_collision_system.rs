use bevy::prelude::{Entity, Query, Res, Time, Transform, Vec2, Vec3, With};

use crate::collision::SolidBodyCollisionQuadTreeHolder;
use crate::models::collider::collider_type::ColliderType;
use crate::models::collider::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::collider::solid_body::SolidBody;
use crate::models::move_direction::MoveDirection;
use crate::models::player::Player;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::util::quad_tree::QuadData;

pub fn solid_body_collision_system(
    time: Res<Time>,
    quad_tree_holder: Res<SolidBodyCollisionQuadTreeHolder>,
    mut solid_body_unit_query: Query<(Entity, &mut Transform, &ColliderType, &mut MoveDirection, &MoveSpeed, &SolidBody)>,
) {
    for (entity, mut transform, unit_size, mut move_direction, move_speed, solid_body) in solid_body_unit_query.iter_mut() {
        let size = match unit_size {
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

            if quad_data.collider_type.is_colliding(&quad_data.position.truncate(), unit_size, &transform.translation.truncate()) {
                let resolution_position = unit_size.get_collision_resolution_position(&transform.translation.truncate(), &quad_data.collider_type, &quad_data.position.truncate());

                collision_resolutions += resolution_position;
                collision_resolutions_counter += 1.0;

                move_direction.direction += (resolution_position.extend(transform.translation.z) - transform.translation).normalize_or_zero();
            }
        }

        if collision_resolutions.length() > 0.0 {
            transform.translation = (collision_resolutions / collision_resolutions_counter).extend(transform.translation.z);

            println!("uhdsa: {:#?}", (collision_resolutions / collision_resolutions_counter).extend(transform.translation.z))
        }
    }
}
