use bevy::prelude::{*};

use crate::{Player};
use crate::models::collider::collider::Collider;
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::items::descriptor::item::Item;
use crate::models::unit_stats_components::{Enemy, UnitSize};
use crate::util::is_colliding::is_colliding;

pub fn item_player_collision_system(
    mut coin_pickup_event: EventWriter<ItemCollisionEvent>,
    mut player_query: Query<(Entity, &Transform, &UnitSize), (With<Collider>, With<Player>, Without<Enemy>)>,
    item_query: Query<(Entity, &Transform, &UnitSize), With<Item>>,
) {
    for (player_entity, player_transform, player_size) in player_query.iter_mut() {
        for (item_entity, item_transform, item_size) in item_query.iter() {
            if is_colliding(
                item_transform.translation,
                item_size.collider_size,
                player_transform.translation,
                player_size.collider_size,
            ) {
                coin_pickup_event.send(ItemCollisionEvent { player_entity, item_entity });
            }
        }
    }
}