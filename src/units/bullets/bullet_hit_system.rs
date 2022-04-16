use bevy::prelude::{Entity, EventReader, EventWriter, Query, With, Without};

use crate::models::bullet::Bullet;
use crate::models::collider::collided_entities::CollidedEntities;
use crate::models::enemy::Enemy;
use crate::models::events::bullet_enemy_collision_event::BulletEnemyCollisionEvent;
use crate::models::events::bullet_stopped_event::BulletStoppedEvent;
use crate::models::events::damaged_event::DamagedEvent;
use crate::models::events::enemy_died_event::EnemyDiedEvent;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::health::Health;
use crate::models::unit_attributes::hit_limit::HitLimit;

/// This system handles the collision between a [Bullet] and an [Enemy].
/// The damage of the bullet gets applied to the health of the enemy.
///
/// The bullet gets checked if the [HitLimit] is not reached and if it is it sends
/// a [BulletEnemyCollisionEvent].
///
/// If an [Enemy] dies from the bullet it will send out an [EnemyDiedEvent].
pub fn bullet_hit_system(
    mut bullet_stopped_event: EventWriter<BulletStoppedEvent>,
    mut damaged_event: EventWriter<DamagedEvent>,
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
            if collided_entities.collisions.len() >= hit_limit.get_total_amount() as usize {
                continue;
            }
        }

        if collided_entities.collisions.contains(&enemy_entity) {
            continue;
        } else {
            collided_entities.collisions.push(enemy_entity);

            enemy_health.damage(bullet_damage.get_total_amount());
            damaged_event.send(DamagedEvent::new(enemy_entity));

            if enemy_health.get_current_health() <= 0.0 {
                enemy_died_event.send(EnemyDiedEvent { enemy_entity })
            }

            if let Some(hit_limit) = hit_limit {
                if collided_entities.collisions.len() >= hit_limit.get_total_amount() as usize {
                    bullet_stopped_event.send(BulletStoppedEvent { bullet_entity: event.bullet_entity });
                    continue;
                }
            }
        }
    }
}