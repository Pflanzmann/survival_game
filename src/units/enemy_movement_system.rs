use bevy::prelude::{Transform, Without};

use crate::{Player, Query, With};
use crate::entities::collision_components::CollisionDirections;
use crate::entities::unit_stats_components::{Enemy, FacingDirection, MoveSpeed};

pub fn enemy_movement_system(
    mut enemies: Query<(&mut Transform, &MoveSpeed, &mut FacingDirection, Option<&CollisionDirections>), (With<Enemy>, Without<Player>)>,
    mut player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_result = match player_query.get_single_mut() {
        Ok(value) => value,
        Err(_) => return,
    };

    for (mut transform, speed, mut enemy_direction, collisions) in enemies.iter_mut() {
        let mut direction = (player_result.translation - transform.translation).normalize();

        if let Some(collisions) = collisions {
            for col_direction in collisions.collisions.iter() {
                direction -= *col_direction / 1.5;

                if direction.length() != 0.0 {
                    direction = direction.normalize();
                }
            }
        }

        transform.translation.x += direction.x * speed.move_speed;
        transform.translation.y += direction.y * speed.move_speed;

        if enemy_direction.facing_direction != direction {
            enemy_direction.facing_direction = direction;
        }
    }
}