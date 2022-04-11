use bevy::app::EventWriter;
use bevy::prelude::{Entity, EventReader, With};

use crate::{Query, Without};
use crate::models::attributes::attribute::*;
use crate::models::attributes::damage::Damage;
use crate::models::attributes::health::Health;
use crate::models::bullet_components::{Bullet, HitLimit};
use crate::models::collider::collided_entities::CollidedEntities;
use crate::models::events::bullet_enemy_collision_event::BulletEnemyCollisionEvent;
use crate::models::events::bullet_stopped_event::BulletStoppedEvent;
use crate::models::events::enemy_died_event::EnemyDiedEvent;
use crate::models::unit_stats_components::Enemy;

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
            }
        };

        let (enemy_entity, mut enemy_health) = match enemy_query.get_mut(event.enemy_entity) {
            Ok(enemy) => enemy,
            Err(_) => {
                continue;
            }
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

            enemy_health.damage(bullet_damage.get_total_amount());
            if enemy_health.get_current_health() <= 0.0 {
                enemy_died_event.send(EnemyDiedEvent { enemy_entity })
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