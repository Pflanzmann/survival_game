use bevy::app::EventWriter;
use bevy::prelude::{Entity, EventReader, With};

use crate::{Damage, Health, Query, Without};
use crate::components::bullet_components::{Bullet, HitLimit};
use crate::components::collision_components::CollidedEntities;
use crate::components::event_components::{BulletEnemyCollisionEvent, BulletStoppedEvent, EnemyDiedEvent};
use crate::components::unit_stats_components::Enemy;

pub fn bullet_hit_system(
    mut bullet_stopped_event: EventWriter<BulletStoppedEvent>,
    mut bullet_enemy_collision_events: EventReader<BulletEnemyCollisionEvent>,
    mut enemy_died_event: EventWriter<EnemyDiedEvent>,
    mut bullets_query: Query<(&mut CollidedEntities, &Damage, Option<&HitLimit>), With<Bullet>>,
    mut enemy_query: Query<(Entity, &mut Health), (With<Enemy>, Without<Bullet>)>,
) {
    for event in bullet_enemy_collision_events.iter() {
        let (mut collided_entities, bullet_damage, hit_limit) = match bullets_query.get_mut(event.bullet_entity) {
            Ok(val) => val,
            Err(_) => {
                continue;
            },
        };

        let (enemy_entity, mut enemy_health) = match enemy_query.get_mut(event.enemy_entity) {
            Ok(enemy) => enemy,
            Err(_) => {
                continue;
            },
        };

        if let Some(hit_limit) = hit_limit {
            if collided_entities.collisions.len() >= hit_limit.hit_limit {
                continue;
            }
        }

        if collided_entities.collisions.contains(&enemy_entity) {
            continue;
        } else {
            collided_entities.collisions.push(enemy_entity);

            if enemy_health.current_health - bullet_damage.damage > 0.0 {
                enemy_health.current_health -= bullet_damage.damage;
            } else {
                enemy_died_event.send(EnemyDiedEvent { enemy_entity });
            }

            if let Some(hit_limit) = hit_limit {
                if collided_entities.collisions.len() >= hit_limit.hit_limit {
                    bullet_stopped_event.send(BulletStoppedEvent { bullet_entity: event.bullet_entity });
                    continue;
                }
            }
        }
    }
}