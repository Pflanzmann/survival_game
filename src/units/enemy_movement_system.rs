use bevy::core::Time;
use bevy::prelude::{Transform, Without};

use crate::{Player, Query, Res, Vec3, With};
use crate::models::unit_attributes::attribute::*;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::collider::collision_directions::CollisionDirections;
use crate::models::unit_stats_components::{Enemy, MoveDirection};

pub fn enemy_movement_system(
    time: Res<Time>,
    mut enemies: Query<(&mut Transform, &MoveSpeed, &mut MoveDirection, Option<&CollisionDirections>), (With<Enemy>, Without<Player>)>,
    mut player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_result = match player_query.get_single_mut() {
        Ok(value) => value,
        Err(_) => return,
    };

    for (mut transform, speed, mut enemy_direction, collisions) in enemies.iter_mut() {
        let mut direction = (player_result.translation - transform.translation).normalize_or_zero();

        if let Some(collisions) = collisions {
            let mut enemies_dodge_direction = Vec3::default();
            for col_direction in collisions.collisions.iter() {
                enemies_dodge_direction -= *col_direction;
            }

            direction += enemies_dodge_direction.normalize_or_zero() * 0.4;
        }

        transform.translation.x += direction.x * speed.get_total_amount() * time.delta_seconds() * 60.0;
        transform.translation.y += direction.y * speed.get_total_amount() * time.delta_seconds() * 60.0;

        if enemy_direction.direction != direction {
            enemy_direction.direction = direction;
        }
    }
}
