use bevy::app::EventReader;
use bevy::prelude::{Entity, EventWriter, With, Without};

use crate::{Player, Query};
use crate::models::attributes::attribute::*;
use crate::models::attributes::damage::Damage;
use crate::models::attributes::health::Health;
use crate::models::events::enemy_died_event::EnemyDiedEvent;
use crate::models::events::player_died_event::PlayerDiedEvent;
use crate::models::events::player_enemy_collision_event::PlayerEnemyCollisionEvent;
use crate::models::unit_stats_components::Enemy;

pub fn player_hit_system(
    mut player_enemy_collision_events: EventReader<PlayerEnemyCollisionEvent>,
    mut enemy_died_event: EventWriter<EnemyDiedEvent>,
    mut player_died_event: EventWriter<PlayerDiedEvent>,
    mut player_query: Query<(Entity, &mut Health, &Damage), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(Entity, &mut Health, &Damage), (With<Enemy>, Without<Player>)>,
) {
    for event in player_enemy_collision_events.iter() {
        let (player_entity, mut player_health, player_damage) = match player_query.get_mut(event.player_entity) {
            Ok(player) => player,
            Err(_) => continue,
        };

        let (enemy_entity, mut enemy_health, enemy_damage) = match enemy_query.get_mut(event.enemy_entity) {
            Ok(enemy) => enemy,
            Err(_) => continue,
        };

        player_health.damage(enemy_damage.get_total_amount());
        if player_health.get_current_health() <= 0.0 {
            player_died_event.send(PlayerDiedEvent { player_entity })
        }

        enemy_health.damage(player_damage.get_total_amount());
        if enemy_health.get_current_health() <= 0.0 {
            enemy_died_event.send(EnemyDiedEvent { enemy_entity })
        }
    }
}