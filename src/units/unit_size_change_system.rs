use bevy::prelude::{Changed, Query, Sprite};

use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::collision::solid_body_collider::SolidBodyCollider;
use crate::models::unit_attributes::unit_size::UnitSize;

pub fn unit_size_change_system(
    mut target_query: Query<(&mut Sprite, &UnitSize, Option<&mut HitBoxCollider>, Option<&mut SolidBodyCollider>), Changed<UnitSize>>
) {
    for (mut sprite, unit_size, hit_box_collider, solid_body_collider) in target_query.iter_mut() {
        sprite.custom_size = Option::Some(unit_size.proportional_unit_size());

        if let Some(mut hit_box_collider) = hit_box_collider {
            match hit_box_collider.collider_type {
                ColliderType::Circle(ref mut radius) => { *radius = unit_size.proportional_unit_size().x / 2.0 }
                ColliderType::Rectangle(ref mut size) => { *size = unit_size.proportional_unit_size() }
            }
        }

        if let Some(mut solid_body_collider) = solid_body_collider {
            match solid_body_collider.collider_type {
                ColliderType::Circle(ref mut radius) => {
                    *radius = unit_size.proportional_unit_size().x / 4.0;

                    solid_body_collider.offset.y = -unit_size.proportional_unit_size().y / 4.0;
                }
                ColliderType::Rectangle(ref mut size) => {
                    *size = unit_size.proportional_unit_size() / 4.0;

                    solid_body_collider.offset.y = -unit_size.proportional_unit_size().y;
                }
            }
        }
    }
}