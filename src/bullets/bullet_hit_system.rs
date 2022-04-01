use bevy::prelude::{Changed, Commands, Entity, EventReader, With};

use crate::{Query, Without};
use crate::components::bullet_components::Bullet;
use crate::components::collision_components::CollidedEntities;
use crate::components::event_components::BulletEnemyCollisionEvent;
use crate::components::unit_stats_components::Enemy;

pub fn bullet_hit_system(
    mut commands: Commands,
    mut bullet_enemy_collision_events: EventReader<BulletEnemyCollisionEvent>,
    mut bullets_query: Query<&mut CollidedEntities, With<Bullet>>,
    enemy_query: Query<Entity, (With<Enemy>, Without<Bullet>)>,
) {
    for event in bullet_enemy_collision_events.iter() {
        println!("event");
        let mut collided_entities = match bullets_query.get_mut(event.bullet_entity) {
            Ok(val) => val,
            Err(_) => {
                println!("not found bullet");
                continue;
            }
        };

        println!("collided");

        let enemy_entities = match enemy_query.get(event.enemy_entity) {
            Ok(enemy) => enemy,
            Err(_) => {
                println!("not found bullet");
                continue;
            }
        };

        if collided_entities.collisions.contains(&enemy_entities) {
            continue;
        } else {
            commands.entity(enemy_entities).despawn();
            collided_entities.collisions.push(enemy_entities);
        }
    }
}