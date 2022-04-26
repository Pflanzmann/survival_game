use bevy::prelude::{Entity, EventWriter, Query, Res, Transform, With, Without};

use crate::util::quad_tree::QuadData;
use crate::collision::SolidBodyCollisionQuadTreeHolder;
use crate::models::collider::collider::Collider;
use crate::models::enemy::Enemy;
use crate::models::events::player_enemy_collision_event::PlayerEnemyCollisionEvent;
use crate::models::player::Player;
use crate::models::unit_size::UnitSize;
use crate::util::is_colliding::is_colliding;

pub fn enemy_player_collision_system(
    mut player_enemy_collision_event: EventWriter<PlayerEnemyCollisionEvent>,
    quad_tree_holder: Res<SolidBodyCollisionQuadTreeHolder>,
    player_query: Query<(Entity, &Transform, &UnitSize), (With<Collider>, With<Player>, Without<Enemy>)>,
) {
    for (player_entity, player_transform, player_size) in player_query.iter() {
        let mut check_entity_list: Vec<QuadData> = Vec::new();
        quad_tree_holder.quad_tree.query_entities(
            &mut check_entity_list,
            &player_transform.translation,
            &player_size.collider_size,
        );

        for quad_data in check_entity_list.iter() {
            if is_colliding(
                quad_data.position,
                quad_data.size,
                player_transform.translation,
                player_size.collider_size,
            ) {
                player_enemy_collision_event.send(PlayerEnemyCollisionEvent { player_entity, enemy_entity: quad_data.entity })
            }
        }
    }
}
