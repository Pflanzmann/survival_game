use bevy::prelude::{Entity, Query, ResMut, Transform, Vec2, With, Without};

use crate::models::bullet::Bullet;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::collision::collision_weight::CollisionWeight;
use crate::models::items::descriptor::item::Item;
use crate::models::player::Player;
use crate::models::resources::item_collision_quad_tree::{ItemCollisionQuadTree, ItemData};
use crate::models::resources::solid_body_quad_tree::{SolidBodyData, SolidBodyQuadTree};
use crate::util::quad_tree::{QuadData, Quadtree};

pub fn calculate_quad_tree_system(
    mut enemy_quad_tree_holder: ResMut<SolidBodyQuadTree>,
    mut item_tree_holder: ResMut<ItemCollisionQuadTree>,
    player_query: Query<&Transform, With<Player>>,
    entity_query: Query<(Entity, &Transform, &ColliderType, &CollisionWeight), (Without<Bullet>, Without<Item>)>,
    item_query: Query<(Entity, &Transform, &ColliderType), With<Item>>,
) {
    for player_position in player_query.iter() {
        let quad_position = player_position.translation.truncate();

        enemy_quad_tree_holder.0 = Quadtree::new(10000.0, 10000.0, quad_position, 0);
        for (entity, transform, collider_type, collision_weight) in entity_query.iter() {
            let size: Vec2 = match collider_type {
                Circle(radius) => Vec2::new(*radius, *radius),
                Rectangle(size) => *size,
            };

            enemy_quad_tree_holder.insert(
                &QuadData {
                    position: transform.translation,
                    size,
                    data: SolidBodyData {
                        entity,
                        collider_type: *collider_type,
                        collision_weight: collision_weight.weight,
                    },
                });
        }

        item_tree_holder.0 = Quadtree::new(10000.0, 10000.0, quad_position, 0);
        for (entity, transform, collider_type) in item_query.iter() {
            let size: Vec2 = match collider_type {
                Circle(radius) => Vec2::new(*radius, *radius),
                Rectangle(size) => *size,
            };

            item_tree_holder.insert(
                &QuadData {
                    position: transform.translation,
                    size,
                    data: ItemData {
                        entity,
                        collider_type: *collider_type,
                    },
                });
        }
    }
}
