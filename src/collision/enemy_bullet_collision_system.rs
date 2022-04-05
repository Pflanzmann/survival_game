use bevy::app::EventWriter;
use bevy::prelude::{Entity, Query, With};

use crate::{Transform, Without};
use crate::entities::bullet_components::Bullet;
use crate::entities::collision_components::Collider;
use crate::entities::events::bullet_enemy_collision_event::BulletEnemyCollisionEvent;
use crate::entities::unit_stats_components::{Enemy, UnitSize};
use crate::util::is_colliding::is_colliding;

pub fn enemy_bullet_collision_system(
    mut bullet_hit_event: EventWriter<BulletEnemyCollisionEvent>,
    enemy_query: Query<(Entity, &Transform, &UnitSize), (With<Collider>, With<Enemy>, Without<Bullet>)>,
    bullet_query: Query<(Entity, &Transform, &UnitSize), (With<Collider>, With<Bullet>, Without<Enemy>)>,
) {
    for (enemy_entity, enemy_transform, enemy_size, ) in enemy_query.iter() {
        for (bullet_entity, bullet_transform, bullet_size) in bullet_query.iter() {
            if is_colliding(
                enemy_transform.translation,
                enemy_size.collider_size,
                bullet_transform.translation,
                bullet_size.collider_size,
            ) {
                bullet_hit_event.send(BulletEnemyCollisionEvent {
                    enemy_entity,
                    bullet_entity,
                })
            }
        }
    }
}