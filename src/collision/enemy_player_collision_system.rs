use bevy::prelude::{Entity, EventWriter, Query, Transform, With, Without};

use crate::models::collider::collider::Collider;
use crate::models::enemy::Enemy;
use crate::models::events::player_enemy_collision_event::PlayerEnemyCollisionEvent;
use crate::models::player::Player;
use crate::models::unit_size::UnitSize;
use crate::util::is_colliding::is_colliding;

pub fn enemy_player_collision_system(
    mut player_enemy_collision_event: EventWriter<PlayerEnemyCollisionEvent>,
    enemy_query: Query<(Entity, &Transform, &UnitSize), (With<Collider>, With<Enemy>, Without<Player>)>,
    player_query: Query<(Entity, &Transform, &UnitSize), (With<Collider>, With<Player>, Without<Enemy>)>,
) {
    for (player_entity, player_transform, player_size) in player_query.iter() {
        for (enemy_entity, enemy_transform, enemy_size) in enemy_query.iter() {
            if is_colliding(
                enemy_transform.translation,
                enemy_size.collider_size,
                player_transform.translation,
                player_size.collider_size,
            ) {
                player_enemy_collision_event.send(PlayerEnemyCollisionEvent { player_entity, enemy_entity })
            }
        }
    }
}
