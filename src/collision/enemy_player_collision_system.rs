use bevy::prelude::{Entity, EventWriter, Query, Res, Transform, Vec2, With};

use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::collider_type::ColliderType::*;
use crate::models::events::player_enemy_collision_event::PlayerEnemyCollisionEvent;
use crate::models::player::Player;
use crate::models::resources::solid_body_collision_quad_tree::SolidBodyCollisionQuadTree;
use crate::util::quad_tree::QuadData;

pub fn enemy_player_collision_system(
    mut player_enemy_collision_event: EventWriter<PlayerEnemyCollisionEvent>,
    quad_tree_holder: Res<SolidBodyCollisionQuadTree>,
    player_query: Query<(Entity, &Transform, &ColliderType), With<Player>>,
) {
    for (player_entity, player_transform, player_collider_type) in player_query.iter() {
        let mut check_entity_list: Vec<QuadData> = Vec::new();

        let size = match player_collider_type {
            Circle(radius) => Vec2::new(*radius, *radius),
            Rectangle(size) => *size,
        };

        quad_tree_holder.query_entities(
            &mut check_entity_list,
            &player_transform.translation,
            &size,
        );

        for quad_data in check_entity_list.iter() {
            if quad_data.collider_type.is_colliding(&quad_data.position.truncate(), player_collider_type, &player_transform.translation.truncate()) {
                player_enemy_collision_event.send(PlayerEnemyCollisionEvent { player_entity, enemy_entity: quad_data.entity })
            }
        }
    }
}
