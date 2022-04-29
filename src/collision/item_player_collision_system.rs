use bevy::prelude::{*};

use crate::collision::ItemCollisionQuadTreeHolder;
use crate::models::collider::collider_type::ColliderType;
use crate::models::collider::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::player::Player;
use crate::util::quad_tree::QuadData;

pub fn item_player_collision_system(
    mut coin_pickup_event: EventWriter<ItemCollisionEvent>,
    mut player_query: Query<(Entity, &Transform, &ColliderType), With<Player>>,
    item_tree_holder: Res<ItemCollisionQuadTreeHolder>,
) {
    for (player_entity, player_transform, player_collider_size) in player_query.iter_mut() {
        let size = match player_collider_size {
            Circle(radius) => Vec2::new(*radius, *radius),
            Rectangle(size) => *size,
        };

        let mut check_entity_list: Vec<QuadData> = Vec::new();
        item_tree_holder.quad_tree.query_entities(
            &mut check_entity_list,
            &player_transform.translation,
            &size,
        );

        for quad_data in check_entity_list.iter() {
            if quad_data.collider_type.is_colliding(&quad_data.position.truncate(), player_collider_size, &player_transform.translation.truncate()) {
                coin_pickup_event.send(ItemCollisionEvent { player_entity, item_entity: quad_data.entity });
            }
        }
    }
}