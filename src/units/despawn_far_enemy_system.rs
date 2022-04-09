use crate::{Commands, Entity, Player, Query, Transform, With, Without};
use crate::models::unit_stats_components::Enemy;

pub fn despawn_far_enemy_system(
    mut commands : Commands,

    mut enemy_query: Query<(Entity, &mut Transform), (With<Enemy>, Without<Player>)>,
    mut player_query: Query<(&mut Transform), (With<Player>, Without<Enemy>)>,
) {
    for mut player_transform in player_query.iter_mut() {
        for (enemy_entity, mut enemy_transform) in enemy_query.iter_mut() {
            let distance_to_player = enemy_transform.translation.distance(player_transform.translation);
            if distance_to_player > (256.0 * 20.0) {
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}