use bevy::prelude::{Commands, Entity, Query, Transform, With, Without};

use crate::models::enemy::Enemy;
use crate::models::player::Player;

pub fn despawn_far_enemy_system(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Transform), (With<Enemy>, Without<Player>)>,
    mut player_query: Query<(&mut Transform), (With<Player>, Without<Enemy>)>,
) {
    for player_transform in player_query.iter_mut() {
        for (enemy_entity, enemy_transform) in enemy_query.iter_mut() {
            let distance_to_player = enemy_transform.translation.distance(player_transform.translation);
            if distance_to_player > (256.0 * 20.0) {
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}