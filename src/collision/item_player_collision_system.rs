use bevy::prelude::{*};

use crate::collision::ItemCollisionQuadTreeHolder;
use crate::util::quad_tree::QuadData;
use crate::models::collider::collider::Collider;
use crate::models::enemy::Enemy;
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::player::Player;
use crate::models::unit_size::UnitSize;
use crate::util::is_colliding::is_colliding;

pub fn item_player_collision_system(
    mut coin_pickup_event: EventWriter<ItemCollisionEvent>,
    mut player_query: Query<(Entity, &Transform, &UnitSize), (With<Collider>, With<Player>, Without<Enemy>)>,
    item_tree_holder: Res<ItemCollisionQuadTreeHolder>,
) {
    for (player_entity, player_transform, player_size) in player_query.iter_mut() {
        let mut check_entity_list: Vec<QuadData> = Vec::new();
        item_tree_holder.quad_tree.query_entities(
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
                coin_pickup_event.send(ItemCollisionEvent { player_entity, item_entity: quad_data.entity });
            }
        }
    }
}