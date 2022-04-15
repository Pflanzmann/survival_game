use bevy::prelude::{Query, Transform, Vec3, With, Without};

use crate::models::collider::collision_directions::CollisionDirections;
use crate::models::enemy::Enemy;
use crate::models::move_direction::MoveDirection;
use crate::models::player::Player;

pub fn enemy_set_move_direction_system(
    mut enemies: Query<(&mut Transform, &mut MoveDirection, Option<&CollisionDirections>), (With<Enemy>, Without<Player>)>,
    mut player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_result = match player_query.get_single_mut() {
        Ok(value) => value,
        Err(_) => return,
    };

    for (transform, mut enemy_direction, collisions) in enemies.iter_mut() {
        let mut direction = (player_result.translation - transform.translation).normalize_or_zero();

        if let Some(collisions) = collisions {
            let mut enemies_dodge_direction = Vec3::default();
            for col_direction in collisions.collisions.iter() {
                enemies_dodge_direction -= *col_direction;
            }

            direction += enemies_dodge_direction.normalize_or_zero() * 0.4;
        }

        if enemy_direction.direction != direction {
            enemy_direction.direction = direction;
        }
    }
}
