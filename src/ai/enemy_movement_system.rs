use bevy::prelude::{Transform, Without};
use crate::{Player, Query, With};
use crate::components::unit_stats_components::{Direction, Enemy, MoveSpeed};

pub fn enemy_movement_system(
    mut enemies: Query<(&mut Transform, &MoveSpeed, &mut Direction), (With<Enemy>, Without<Player>)>,
    mut player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_result = player_query.get_single_mut();
    if player_result.is_err(){
        return;
    }

    let player_transform = player_result.unwrap();

    for (mut transform, speed, mut enemy_direction) in enemies.iter_mut() {
        let direction = (player_transform.translation - transform.translation).normalize();

        transform.translation.x += direction.x * speed.move_speed;
        transform.translation.y += direction.y * speed.move_speed;
        enemy_direction.direction = direction;
    }
}