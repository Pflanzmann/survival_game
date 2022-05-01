use bevy::prelude::{Entity, EventReader, EventWriter, Query, With, Without};

use crate::models::enemy::Enemy;
use crate::models::events::enemy_died_event::EnemyDiedEvent;
use crate::models::events::player_died_event::PlayerDiedEvent;
use crate::models::events::player_enemy_collision_event::PlayerEnemyCollisionEvent;
use crate::models::player::Player;
use crate::models::unit_attributes::attribute::*;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::health::Health;
use crate::models::unit_attributes::meele_attack_speed::MeeleAttackSpeed;

pub fn player_hit_system(
    mut player_enemy_collision_events: EventReader<PlayerEnemyCollisionEvent>,
    mut enemy_died_event: EventWriter<EnemyDiedEvent>,
    mut player_died_event: EventWriter<PlayerDiedEvent>,
    mut player_query: Query<(Entity, Option<&mut Health>, &Damage, &mut MeeleAttackSpeed), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(Entity, Option<&mut Health>, &Damage, &mut MeeleAttackSpeed), (With<Enemy>, Without<Player>)>,
) {
    for event in player_enemy_collision_events.iter() {
        let (player_entity, player_health, player_damage, mut player_meele_attack_speed) = match player_query.get_mut(event.player_entity) {
            Ok(player) => player,
            Err(_) => continue,
        };

        let (enemy_entity, enemy_health, enemy_damage, mut enemy_meele_attack_speed) = match enemy_query.get_mut(event.enemy_entity) {
            Ok(enemy) => enemy,
            Err(_) => continue,
        };

        if enemy_meele_attack_speed.reload_timer <= 0.0 {
            enemy_meele_attack_speed.reload_timer = enemy_meele_attack_speed.get_total_amount();

            if let Some(mut player_health) = player_health {
                player_health.damage(enemy_damage.get_total_amount());
                if player_health.get_current_health() <= 0.0 {
                    player_died_event.send(PlayerDiedEvent { player_entity })
                }
            }
        }

        if player_meele_attack_speed.reload_timer <= 0.0 {
            player_meele_attack_speed.reload_timer = player_meele_attack_speed.get_total_amount();

            if let Some(mut enemy_health) = enemy_health {
                enemy_health.damage(player_damage.get_total_amount());
                if enemy_health.get_current_health() <= 0.0 {
                    enemy_died_event.send(EnemyDiedEvent { enemy_entity })
                }
            }
        }
    }
}