use bevy::prelude::{Entity, EventWriter, Query, Res, Transform, Vec2, With};

use crate::models::collision::collider_type::ColliderType::*;
use crate::models::collision::solid_body_collider::SolidBodyCollider;
use crate::models::events::player_enemy_collision_event::PlayerEnemyCollisionEvent;
use crate::models::player::Player;
use crate::models::resources::solid_body_quad_tree::{SolidBodyData, SolidBodyQuadTree};
use crate::util::quad_tree::QuadData;

pub fn enemy_player_collision_system(
    mut player_enemy_collision_event: EventWriter<PlayerEnemyCollisionEvent>,
    quad_tree: Res<SolidBodyQuadTree>,
    player_query: Query<(Entity, &Transform, &SolidBodyCollider), With<Player>>,
) {
    for (player_entity, player_transform, player_solid_body_collider) in player_query.iter() {
        let mut check_entity_list: Vec<QuadData<SolidBodyData>> = Vec::new();

        let size = match player_solid_body_collider.collider_type {
            Circle(radius) => Vec2::new(radius, radius),
            Rectangle(size) => size,
        };

        let self_position = player_transform.translation + player_solid_body_collider.offset;

        quad_tree.query_entities(
            &mut check_entity_list,
            &self_position,
            &size,
        );

        for quad_data in check_entity_list.iter() {
            if quad_data.data.collider_type.is_colliding(&quad_data.position.truncate(), &player_solid_body_collider.collider_type, &self_position.truncate()) {
                player_enemy_collision_event.send(PlayerEnemyCollisionEvent { player_entity, enemy_entity: quad_data.data.entity })
            }
        }
    }
}
