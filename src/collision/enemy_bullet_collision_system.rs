use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, With};
use bevy::sprite::collide_aabb::collide;

use crate::{Player, Transform, Without};
use crate::components::unit_stats_components::{Enemy, UnitSize};
use crate::components::bullet_components::Bullet;
use crate::collision::collision_components::Collider;
use crate::components::event_components::EnemyDiedEvent;

pub fn enemy_bullet_collision_system(
    mut commands: Commands,
    mut enemy_died_event: EventWriter<EnemyDiedEvent>,
    enemy_query: Query<(Entity, &Transform, &UnitSize), (With<Collider>, With<Enemy>, Without<Bullet>)>,
    bullet_query: Query<(&Transform, &UnitSize), (With<Collider>, With<Bullet>, Without<Enemy>)>,
) {
    for (enemy_entity, enemy_transform, enemy_size) in enemy_query.iter() {
        for (bullet_transform, bullet_size) in bullet_query.iter() {
            if collide(
                enemy_transform.translation,
                enemy_size.collider_size,
                bullet_transform.translation,
                bullet_size.collider_size,
            ).is_some() {
                commands.entity(enemy_entity).despawn();

                enemy_died_event.send(EnemyDiedEvent{death_position: enemy_transform.translation});
            }
        }
    }
}