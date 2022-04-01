use bevy::app::EventWriter;
use bevy::prelude::{Entity, Query, With};
use bevy::sprite::collide_aabb::collide;

use crate::{Transform, Without};
use crate::components::bullet_components::Bullet;
use crate::components::collision_components::Collider;
use crate::components::event_components::BulletEnemyCollisionEvent;
use crate::components::unit_stats_components::{Enemy, UnitSize};

pub fn enemy_bullet_collision_system(
    mut bullet_hit_event: EventWriter<BulletEnemyCollisionEvent>,
    enemy_query: Query<(Entity, &Transform, &UnitSize), (With<Collider>, With<Enemy>, Without<Bullet>)>,
    bullet_query: Query<(Entity, &Transform, &UnitSize), (With<Collider>, With<Bullet>, Without<Enemy>)>,
) {
    let mut counter = 0;
    for (enemy_entity, enemy_transform, enemy_size, ) in enemy_query.iter() {
        for (bullet_entity, bullet_transform, bullet_size) in bullet_query.iter() {
            if collide(
                enemy_transform.translation,
                enemy_size.collider_size,
                bullet_transform.translation,
                bullet_size.collider_size,
            ).is_some() {
                counter += 1;
                bullet_hit_event.send(BulletEnemyCollisionEvent {
                    enemy_entity,
                    bullet_entity,
                })
            }
        }
    }

    if counter != 0 {
        println!("counter: {}", counter)
    }
}