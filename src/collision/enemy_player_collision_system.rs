use bevy::prelude::{Commands, Entity, Query, With};
use bevy::sprite::collide_aabb::collide;

use crate::{Player, Transform, Without};
use crate::components::unit_stats_components::{Damage, Enemy, Health, ColliderSize};
use crate::collision::collision_components::Collider;

pub fn enemy_player_collision_system(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform, &ColliderSize, &Damage), (With<Collider>, With<Enemy>, Without<Player>)>,
    mut player_query: Query<(Entity, &Transform, &ColliderSize, &mut Health), (With<Collider>, With<Player>, Without<Enemy>)>,
) {
    let player_result = player_query.get_single_mut();
    if player_result.is_err(){
        return;
    }

    let (player_entity, player_transform, player_size, mut player_health) = player_result.unwrap();

    for (enemy_entity, enemy_transform, enemy_size, enemy_damage) in enemy_query.iter() {
        if collide(
            enemy_transform.translation,
            enemy_size.collider_size,
            player_transform.translation,
            player_size.collider_size,
        ).is_some() {
            //commands.entity(enemy_entity).despawn()
            player_health.health -= enemy_damage.damage.clone();
            if player_health.health <= 0.0 {
                commands.entity(player_entity).despawn()
            }
        }
    }
}