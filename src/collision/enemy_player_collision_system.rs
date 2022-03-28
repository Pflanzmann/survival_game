use bevy::prelude::{Commands, Entity, Query, With};
use bevy::sprite::collide_aabb::collide;

use crate::{Player, Transform, Without};
use crate::ai::ai_components::{EnemyAi, Size};
use crate::collision::collision_components::Collider;

pub fn enemy_player_collision_system(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform, &Size), (With<Collider>, With<EnemyAi>, Without<Player>)>,
    player_query: Query<(&Transform, &Size), (With<Collider>, With<Player>, Without<EnemyAi>)>,
) {
    let mut test_objects = Vec::<(Entity, &Transform, &Size)>::new();
    let (player_transform, player_size) = player_query.get_single().unwrap();

    for (test_entity, test_transform, test_size) in enemy_query.iter() {
        test_objects.push((test_entity, test_transform, test_size))
    }

    for (enemy_entity, enemy_transform, enemy_size) in test_objects.iter() {
        if collide(
            enemy_transform.translation,
            enemy_size.size,
            player_transform.translation,
            player_size.size,
        ).is_some() {
            commands.entity(*enemy_entity).despawn()
        }
    }
}