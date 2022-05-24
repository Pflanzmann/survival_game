use bevy::prelude::{Commands, Entity, Query, Transform, With};

use crate::models::enemy::Enemy;
use crate::models::player::Player;

pub fn despawn_far_enemy_system(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    for player_transform in player_query.iter() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let distance_to_player = enemy_transform.translation.truncate().distance(player_transform.translation.truncate());
            if distance_to_player > (256.0 * 20.0) {
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}