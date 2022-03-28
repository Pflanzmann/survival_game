use bevy::prelude::{Transform, Without};
use crate::{Player, Query, With};
use crate::ai::ai_components::{Direction, EnemyAi, Speed};

pub fn enemy_movement_system(
    mut enemies: Query<(&mut Transform, &Speed, &mut Direction), (With<EnemyAi>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<EnemyAi>)>,
) {
    let mut player_transform = player.get_single().unwrap();

    for (mut transform, speed, mut enemy_direction) in enemies.iter_mut() {
        let direction = (player_transform.translation - transform.translation).normalize();

        transform.translation.x += direction.x * speed.speed;
        transform.translation.y += direction.y * speed.speed;
        enemy_direction.direction = direction;
    }
}