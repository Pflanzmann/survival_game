use bevy::prelude::{Query, Transform, Vec3};

use crate::models::collider::collision_directions::CollisionDirections;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::move_speed::MoveSpeed;

pub fn enemy_collision_dodge_system(
    mut enemies: Query<(&mut Transform, &MoveSpeed, &CollisionDirections)>,
) {
    for (mut transform, move_speed, collisions) in enemies.iter_mut() {
        let mut enemies_dodge_direction = Vec3::default();
        for col_direction in collisions.collisions.iter() {
            enemies_dodge_direction -= *col_direction;
        }

        transform.translation += enemies_dodge_direction.normalize_or_zero() * 0.4 * move_speed.get_total_amount();
    }
}
