use bevy::prelude::{Query, Res, Time, Transform, Vec3};

use crate::models::collider::collision_directions::CollisionDirections;
use crate::models::move_direction::MoveDirection;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::move_speed::MoveSpeed;

pub fn enemy_collision_dodge_system(
    time: Res<Time>,
    mut enemies: Query<(&mut Transform, &MoveSpeed, &MoveDirection, &mut CollisionDirections)>,
) {
    for (mut transform, move_speed, direction, mut collisions) in enemies.iter_mut() {
        if collisions.collisions.len() == 0 {
            continue;
        }

        let mut enemies_dodge_direction = Vec3::default();
        for col_direction in collisions.collisions.iter() {
            enemies_dodge_direction -= *col_direction;
        }

        let new_direction = (direction.direction + enemies_dodge_direction).normalize_or_zero();


        // transform.translation += ((-direction.direction + new_direction.normalize_or_zero()) * move_speed.get_total_amount() * time.delta_seconds() * 60.0) * 0.5;
    }
}
