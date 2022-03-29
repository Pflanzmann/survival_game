use bevy::prelude::{Commands, Entity, Query, With};
use bevy::sprite::collide_aabb::collide;

use crate::{Player, Transform, Without};
use crate::collision::collision_components::Collider;
use crate::components::unit_stats_components::{Damage, Enemy, Health, UnitSize};

pub fn enemy_player_collision_system(
    mut commands: Commands,
    enemy_query: Query<(&Transform, &UnitSize, &Damage), (With<Collider>, With<Enemy>, Without<Player>)>,
    mut player_query: Query<(Entity, &Transform, &UnitSize, &mut Health), (With<Collider>, With<Player>, Without<Enemy>)>,
) {
    for (player_entity, player_transform, player_size, mut player_health) in player_query.iter_mut() {
        for (enemy_transform, enemy_size, enemy_damage) in enemy_query.iter() {
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
}