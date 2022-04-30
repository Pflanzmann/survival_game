use bevy::prelude::{Changed, Query, Sprite};

use crate::models::collider::collider_type::ColliderType;
use crate::models::unit_size::UnitSize;

pub fn fit_sprite_to_size_system(
    mut target_query: Query<(&mut Sprite, &UnitSize, Option<&mut ColliderType>), Changed<UnitSize>>
) {
    for (mut sprite, unit_size, mut collider_type) in target_query.iter_mut() {
        sprite.custom_size = Option::Some(unit_size.collider_size);

        if let Some(mut collider_type) = collider_type {
            match *collider_type {
                ColliderType::Circle(ref mut radius) => { *radius = unit_size.collider_size.x / 2.0 }
                ColliderType::Rectangle(ref mut size) => { *size = unit_size.collider_size }
            }
        }
    }
}