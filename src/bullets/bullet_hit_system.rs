use bevy::app::EventWriter;
use bevy::prelude::{Commands, Entity, EventReader, With};

use crate::{Query, Transform, Without};
use crate::components::bullet_components::Bullet;
use crate::components::collision_components::CollidedEntities;
use crate::components::event_components::{BulletEnemyCollisionEvent, EnemyDiedEvent};
use crate::components::unit_stats_components::Enemy;

pub fn bullet_hit_system(
    mut commands: Commands,
    mut bullet_enemy_collision_events: EventReader<BulletEnemyCollisionEvent>,
    mut enemy_died_event: EventWriter<EnemyDiedEvent>,
    mut bullets_query: Query<&mut CollidedEntities, With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), (With<Enemy>, Without<Bullet>)>,
) {
    for event in bullet_enemy_collision_events.iter() {
        let mut collided_entities = match bullets_query.get_mut(event.bullet_entity) {
            Ok(val) => val,
            Err(_) => continue,
        };

        let (enemy_entities, transform) = match enemy_query.get(event.enemy_entity) {
            Ok(enemy) => enemy,
            Err(_) => continue,
        };

        if collided_entities.collisions.contains(&enemy_entities) {
            continue;
        } else {
            collided_entities.collisions.push(enemy_entities);
            commands.entity(enemy_entities).despawn();
            enemy_died_event.send(EnemyDiedEvent { death_position: transform.translation })
        }
    }
}