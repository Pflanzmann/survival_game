use bevy::prelude::{Commands, Entity, Query, With};

use crate::{Player, Transform, Without};
use crate::entities::collider::collider::Collider;
use crate::entities::unit_stats_components::{Damage, Enemy, Health, UnitSize};
use crate::util::is_colliding::is_colliding;

pub fn enemy_player_collision_system(
    mut commands: Commands,
    enemy_query: Query<(&Transform, &UnitSize, &Damage), (With<Collider>, With<Enemy>, Without<Player>)>,
    mut player_query: Query<(Entity, &Transform, &UnitSize, &mut Health), (With<Collider>, With<Player>, Without<Enemy>)>,
) {
    for (player_entity, player_transform, player_size, mut player_health) in player_query.iter_mut() {
        for (enemy_transform, enemy_size, enemy_damage) in enemy_query.iter() {
            if is_colliding(
                enemy_transform.translation,
                enemy_size.collider_size,
                player_transform.translation,
                player_size.collider_size,
            ) {
                player_health.current_health -= enemy_damage.get_damage();

                if player_health.current_health <= 0.0 {
                    commands.entity(player_entity).despawn()
                }
            }
        }
    }
}
