use bevy::ecs::query::QueryEntityError;
use bevy::prelude::{Children, Commands, Entity, Query, With, World};
use bevy::sprite::collide_aabb::collide;

use crate::{HealthBar, Player, Transform, Without};
use crate::components::collision_components::Collider;
use crate::components::unit_stats_components::{Damage, Enemy, Health, UnitSize};

pub fn enemy_player_collision_system(
    mut commands: Commands,
    enemy_query: Query<(&Transform, &UnitSize, &Damage), (With<Collider>, With<Enemy>, Without<Player>)>,
    mut player_query: Query<(Entity, &Transform, &UnitSize, &mut Health, &mut Children), (With<Collider>, With<Player>, Without<Enemy>)>,
    mut children_query: Query<&mut Transform, (With<HealthBar>, Without<Player>, Without<Enemy>)>,
) {
    for (player_entity, player_transform, player_size, mut player_health, mut children) in player_query.iter_mut() {
        for (enemy_transform, enemy_size, enemy_damage) in enemy_query.iter() {
            if collide(
                enemy_transform.translation,
                enemy_size.collider_size,
                player_transform.translation,
                player_size.collider_size,
            ).is_some() {
                player_health.current_health -= enemy_damage.damage.clone();

                if player_health.current_health <= 0.0 {
                    commands.entity(player_entity).despawn()
                }
            }
        }
    }
}
