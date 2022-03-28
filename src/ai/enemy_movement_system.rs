use bevy::prelude::{Transform, Without};
use crate::{Player, Query, With};
use crate::ai::ai_components::EnemyAi;

pub fn enemy_movement_system(
    mut enemies: Query<&mut Transform, (With<EnemyAi>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<EnemyAi>)>,
) {
    let mut player_transform = player.get_single().unwrap();

    for mut transform in enemies.iter_mut() {
        let direction = (player_transform.translation - transform.translation).normalize();

        transform.translation.x += direction.x * 10.0;
        transform.translation.y += direction.y * 10.0;
    }
}