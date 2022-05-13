use bevy::prelude::{Entity, EventReader, EventWriter, Query, Res, Time, With, Without};

use crate::models::damaged_entities::{DamagedEntities, DamagedEntity};
use crate::models::enemy::Enemy;
use crate::models::events::bullet_stopped_event::BulletStoppedEvent;
use crate::models::events::damaged_event::DamagedEvent;
use crate::models::events::enemy_collision_event::EnemyCollisionEvent;
use crate::models::events::target_died_event::TargetDiedEvent;
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
pub fn hit_system(
    time: Res<Time>,
    mut bullet_stopped_event: EventWriter<BulletStoppedEvent>,
    mut damaged_event: EventWriter<DamagedEvent>,
    mut enemy_collision_events: EventReader<EnemyCollisionEvent>,
    mut target_died_event: EventWriter<TargetDiedEvent>,
    mut hitting_query: Query<(&mut DamagedEntities, &Damage, Option<&mut HitLimit>), Without<Enemy>>,
    mut target_query: Query<(Entity, &mut Health), With<Enemy>>,
) {
    for event in enemy_collision_events.iter() {
        let (mut damaged_entities, damage, hit_limit) = match hitting_query.get_mut(event.source_entity) {
            Ok(val) => val,
            Err(_) => continue,
        };

        let (target_entity, mut enemy_health) = match target_query.get_mut(event.target_entity) {
            Ok(enemy) => enemy,
            Err(_) => continue,
        };

        if let Some(hit_limit) = hit_limit.as_ref() {
            if hit_limit.hit_counter >= hit_limit.get_total_amount() as i32 {
                continue;
            }
        }

        let damaged_entity = DamagedEntity::new(target_entity, time.seconds_since_startup());
        if damaged_entities.contains(&damaged_entity) {
            continue;
        }

        damaged_entities.push(damaged_entity);

        enemy_health.damage(damage.get_total_amount());
        damaged_event.send(DamagedEvent::new(target_entity));

        if enemy_health.get_current_health() <= 0.0 {
            target_died_event.send(TargetDiedEvent { target_entity })
        }

        if let Some(mut hit_limit) = hit_limit {
            hit_limit.hit_counter += 1;

            if hit_limit.hit_counter >= hit_limit.get_total_amount() as i32 {
                bullet_stopped_event.send(BulletStoppedEvent { bullet_entity: event.source_entity });
                continue;
            }
        }
    }
}