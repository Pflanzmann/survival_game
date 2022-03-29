use bevy::prelude::{Transform, Without};
use crate::{Player, Query, With};
use crate::components::unit_stats_components::{Direction, Enemy, Speed};

pub fn enemy_movement_system(
    mut enemies: Query<(&mut Transform, &Speed, &mut Direction), (With<Enemy>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player.get_single().unwrap();

    for (mut transform, speed, mut enemy_direction) in enemies.iter_mut() {
        let direction = (player_transform.translation - transform.translation).normalize();

        transform.translation.x += direction.x * speed.speed;
        transform.translation.y += direction.y * speed.speed;
        enemy_direction.direction = direction;
    }
}