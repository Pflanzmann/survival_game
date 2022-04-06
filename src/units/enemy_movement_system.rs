use bevy::core::Time;
use bevy::prelude::{Transform, Without};

use crate::{Player, Query, Res, With};
use crate::models::collider::collision_directions::CollisionDirections;
use crate::models::unit_stats_components::{Enemy, MoveDirection, MoveSpeed};

pub fn enemy_movement_system(
    time : Res<Time>,

    mut enemies: Query<(&mut Transform, &MoveSpeed, &mut MoveDirection, Option<&CollisionDirections>), (With<Enemy>, Without<Player>)>,
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

        transform.translation.x += direction.x * speed.move_speed * time.delta_seconds() * 60.0;
        transform.translation.y += direction.y * speed.move_speed * time.delta_seconds() * 60.0;

        if enemy_direction.direction != direction {
            enemy_direction.direction = direction;
        }
    }
}
