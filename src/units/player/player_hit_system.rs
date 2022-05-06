use bevy::prelude::{Entity, EventReader, EventWriter, Query, Res, Time, With, Without};

use crate::models::collision::collided_entities::{DamagedEntities, DamagedEntity};
use crate::models::enemy::Enemy;
use crate::models::events::enemy_died_event::EnemyDiedEvent;
use crate::models::events::player_died_event::PlayerDiedEvent;
use crate::models::events::player_enemy_collision_event::PlayerEnemyCollisionEvent;
use crate::models::player::Player;
use crate::models::unit_attributes::attribute::*;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::health::Health;

pub fn player_hit_system(
    time: Res<Time>,
    mut player_enemy_collision_events: EventReader<PlayerEnemyCollisionEvent>,
    mut enemy_died_event: EventWriter<EnemyDiedEvent>,
    mut player_died_event: EventWriter<PlayerDiedEvent>,
    mut player_query: Query<(Entity, Option<&mut Health>, &Damage, &mut DamagedEntities), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(Entity, Option<&mut Health>, &Damage, &mut DamagedEntities), (With<Enemy>, Without<Player>)>,
) {
    for event in player_enemy_collision_events.iter() {
        let (player_entity, player_health, player_damage, mut player_damaged_entities) = match player_query.get_mut(event.player_entity) {
            Ok(player) => player,
            Err(_) => continue,
        };

        let (enemy_entity, enemy_health, enemy_damage, mut enemy_damaged_entities) = match enemy_query.get_mut(event.enemy_entity) {
            Ok(enemy) => enemy,
            Err(_) => continue,
        };

        let damaged_entity = DamagedEntity::new(enemy_entity, time.seconds_since_startup());
        if !enemy_damaged_entities.contains(&damaged_entity) {
            enemy_damaged_entities.push(damaged_entity);

            if let Some(mut player_health) = player_health {
                player_health.damage(enemy_damage.get_total_amount());
                if player_health.get_current_health() <= 0.0 {
                    player_died_event.send(PlayerDiedEvent { player_entity })
                }
            }
        }

        let damaged_entity = DamagedEntity::new(enemy_entity, time.seconds_since_startup());
        if !player_damaged_entities.contains(&damaged_entity) {
            player_damaged_entities.push(damaged_entity);

            if let Some(mut enemy_health) = enemy_health {
                enemy_health.damage(player_damage.get_total_amount());
                if enemy_health.get_current_health() <= 0.0 {
                    enemy_died_event.send(EnemyDiedEvent { enemy_entity })
                }
            }
        }
    }
}