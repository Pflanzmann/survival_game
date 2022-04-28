use bevy::prelude::{Entity, EventWriter, Query, Res, Transform, Vec2, With, Without};

use crate::collision::SolidBodyCollisionQuadTreeHolder;
use crate::models::collider::collider::Collider;
use crate::models::collider::collider_type::ColliderType;
use crate::models::collider::collider_type::ColliderType::*;
use crate::models::enemy::Enemy;
use crate::models::events::player_enemy_collision_event::PlayerEnemyCollisionEvent;
use crate::models::player::Player;
use crate::models::unit_size::UnitSize;
use crate::util::is_colliding::is_colliding;
use crate::util::quad_tree::QuadData;

pub fn enemy_player_collision_system(
    mut player_enemy_collision_event: EventWriter<PlayerEnemyCollisionEvent>,
    quad_tree_holder: Res<SolidBodyCollisionQuadTreeHolder>,
    player_query: Query<(Entity, &Transform, &ColliderType), (With<Collider>, With<Player>, Without<Enemy>)>,
) {
    for (player_entity, player_transform, player_size) in player_query.iter() {
        let mut check_entity_list: Vec<QuadData> = Vec::new();

        let size = match player_size {
            Circle(radius) => Vec2::new(*radius, *radius),
            Rectangle(size) => *size,
        };

        quad_tree_holder.quad_tree.query_entities(
            &mut check_entity_list,
            &player_transform.translation,
            &size,
        );

        for quad_data in check_entity_list.iter() {
            if quad_data.collider_type.is_colliding(&quad_data.position.truncate(), player_size, &player_transform.translation.truncate()) {
                player_enemy_collision_event.send(PlayerEnemyCollisionEvent { player_entity, enemy_entity: quad_data.entity })
            }
        }
    }
}
