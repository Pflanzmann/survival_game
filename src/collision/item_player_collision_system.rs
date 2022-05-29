use bevy::prelude::{*};

use crate::models::collision::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::collision::item_collider::ItemCollider;
use crate::models::collision::solid_body_collider::SolidBodyCollider;
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::resources::collision::item_collision_quad_tree::{ItemCollisionQuadTree, ItemData};
use crate::util::quad_tree::QuadData;

pub fn item_player_collision_system(
    mut item_collision_event: EventWriter<ItemCollisionEvent>,
    mut player_query: Query<(Entity, &Transform, &SolidBodyCollider), With<ItemCollider>>,
    item_quad_tree: Res<ItemCollisionQuadTree>,
) {
    for (player_entity, player_transform, player_collider_size) in player_query.iter_mut() {
        let size = match player_collider_size.collider_type {
            Circle(radius) => Vec2::new(radius, radius),
            Rectangle(size) => size,
        };

        let mut check_entity_list: Vec<QuadData<ItemData>> = Vec::new();
        item_quad_tree.query_entities(
            &mut check_entity_list,
            &player_transform.translation.truncate(),
            &size,
        );

        for quad_data in check_entity_list.iter() {
            if quad_data.data.collider_type.is_colliding(&quad_data.position, &player_collider_size.collider_type, &player_transform.translation.truncate()) {
                item_collision_event.send(ItemCollisionEvent { target_entity: player_entity, source_entity: quad_data.data.entity });
            }
        }
    }
}