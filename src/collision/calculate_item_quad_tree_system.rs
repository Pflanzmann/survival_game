use bevy::prelude::{Entity, Query, ResMut, Transform, Vec2, With};

use crate::models::collision::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::items::descriptor::item::Item;
use crate::models::player::Player;
use crate::models::resources::collision::item_collision_quad_tree::{ItemCollisionQuadTree, ItemData};
use crate::util::quad_tree::{QuadData, Quadtree};

pub fn calculate_item_quad_tree_system(
    mut item_tree: ResMut<ItemCollisionQuadTree>,
    player_query: Query<&Transform, With<Player>>,
    item_query: Query<(Entity, &Transform, &HitBoxCollider), With<Item>>,
) {
    for player_position in player_query.iter() {
        let quad_position = player_position.translation.truncate();

        item_tree.0 = Quadtree::new(10000.0, 10000.0, quad_position, 0);
        for (entity, transform, hit_box_collider) in item_query.iter() {
            let size: Vec2 = match hit_box_collider.collider_type {
                Circle(radius) => Vec2::new(radius, radius),
                Rectangle(size) => size,
            };

            item_tree.insert(
                &QuadData {
                    position: transform.translation.truncate(),
                    size,
                    data: ItemData {
                        entity,
                        collider_type: hit_box_collider.collider_type,
                    },
                });
        }
    }
}
