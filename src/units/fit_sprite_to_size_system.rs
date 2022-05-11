use bevy::prelude::{Changed, Query, Sprite};
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::hit_box_collider::HitBoxCollider;

use crate::models::unit_size::UnitSize;

pub fn fit_sprite_to_size_system(
    mut target_query: Query<(&mut Sprite, &UnitSize, Option<&mut HitBoxCollider>), Changed<UnitSize>>
) {
    for (mut sprite, unit_size, hit_box_collider) in target_query.iter_mut() {
        sprite.custom_size = Option::Some(unit_size.collider_size);

        if let Some(mut hit_box_collider) = hit_box_collider {
            match hit_box_collider.collider_type {
                ColliderType::Circle(ref mut radius) => { *radius = unit_size.collider_size.x / 2.0 }
                ColliderType::Rectangle(ref mut size) => { *size = unit_size.collider_size }
            }
        }
    }
}