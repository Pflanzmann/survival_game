use bevy::prelude::{Commands, Entity, Query, With};
use bevy::sprite::collide_aabb::collide;

use crate::{Player, Transform, Without};
use crate::components::unit_stats_components::{Enemy, Size};
use crate::collision::collision_components::Collider;

pub fn enemy_player_collision_system(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform, &Size), (With<Collider>, With<Enemy>, Without<Player>)>,
    player_query: Query<(&Transform, &Size), (With<Collider>, With<Player>, Without<Enemy>)>,
) {
    let (player_transform, player_size) = player_query.get_single().unwrap();

    for (enemy_entity, enemy_transform, enemy_size) in enemy_query.iter() {
        if collide(
            enemy_transform.translation,
            enemy_size.size,
            player_transform.translation,
            player_size.size,
        ).is_some() {
            commands.entity(enemy_entity).despawn()
        }
    }
}