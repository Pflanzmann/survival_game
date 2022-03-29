use bevy::prelude::{Commands, Entity, Query, With};
use bevy::sprite::collide_aabb::collide;

use crate::{Player, Transform, Without};
use crate::components::unit_stats_components::{Enemy, Size};
use crate::components::bullet_components::Bullet;
use crate::collision::collision_components::Collider;

pub fn enemy_bullet_collision_system(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform, &Size), (With<Collider>, With<Enemy>, Without<Bullet>)>,
    bullet_query: Query<(&Transform, &Size), (With<Collider>, With<Bullet>, Without<Enemy>)>,
) {
    for (enemy_entity, enemy_transform, enemy_size) in enemy_query.iter() {
        for (bullet_transform, bullet_size) in bullet_query.iter() {
            if collide(
                enemy_transform.translation,
                enemy_size.size,
                bullet_transform.translation,
                bullet_size.size,
            ).is_some() {
                commands.entity(enemy_entity).despawn()
            }
        }
    }
}