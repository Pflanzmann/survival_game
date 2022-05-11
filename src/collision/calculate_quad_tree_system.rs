use bevy::prelude::{Entity, GlobalTransform, Query, ResMut, Vec2, With};

use crate::models::collision::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::collision::collider_weight::ColliderWeight;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::collision::solid_body_collider::SolidBodyCollider;
use crate::models::enemy::Enemy;
use crate::models::items::descriptor::item::Item;
use crate::models::player::Player;
use crate::models::resources::hit_box_quad_tree::{HitBoxData, HitBoxQuadTree};
use crate::models::resources::item_collision_quad_tree::{ItemCollisionQuadTree, ItemData};
use crate::models::resources::solid_body_quad_tree::{SolidBodyData, SolidBodyQuadTree};
use crate::util::quad_tree::{QuadData, Quadtree};

pub fn calculate_quad_tree_system(
    mut solid_body_tree: ResMut<SolidBodyQuadTree>,
    mut item_tree: ResMut<ItemCollisionQuadTree>,
    mut hit_box_tree: ResMut<HitBoxQuadTree>,
    player_query: Query<&GlobalTransform, With<Player>>,
    hit_box_collider_query: Query<(Entity, &GlobalTransform, &HitBoxCollider), With<Enemy>>,
    solid_body_query: Query<(Entity, &GlobalTransform, &SolidBodyCollider, &ColliderWeight)>,
    item_query: Query<(Entity, &GlobalTransform, &HitBoxCollider), With<Item>>,
) {
    for player_position in player_query.iter() {
        let quad_position = player_position.translation.truncate();

        hit_box_tree.0 = Quadtree::new(10000.0, 10000.0, quad_position, 0);
        for (entity, transform, hit_box_collider) in hit_box_collider_query.iter() {
            let size: Vec2 = match hit_box_collider.collider_type {
                Circle(radius) => Vec2::new(radius, radius),
                Rectangle(size) => size,
            };

            hit_box_tree.insert(
                &QuadData {
                    position: transform.translation,
                    size,
                    data: HitBoxData {
                        entity,
                        collider_type: hit_box_collider.collider_type,
                    },
                });
        }

        solid_body_tree.0 = Quadtree::new(10000.0, 10000.0, quad_position, 0);
        for (entity, transform, solid_body_collider, collision_weight) in solid_body_query.iter() {
            let size: Vec2 = match solid_body_collider.collider_type {
                Circle(radius) => Vec2::new(radius, radius),
                Rectangle(size) => size,
            };

            solid_body_tree.insert(
                &QuadData {
                    position: transform.translation + solid_body_collider.offset,
                    size,
                    data: SolidBodyData {
                        entity,
                        collider_type: solid_body_collider.collider_type,
                        collision_weight: collision_weight.weight,
                    },
                });
        }

        item_tree.0 = Quadtree::new(10000.0, 10000.0, quad_position, 0);
        for (entity, transform, hit_box_collider) in item_query.iter() {
            let size: Vec2 = match hit_box_collider.collider_type {
                Circle(radius) => Vec2::new(radius, radius),
                Rectangle(size) => size,
            };

            item_tree.insert(
                &QuadData {
                    position: transform.translation,
                    size,
                    data: ItemData {
                        entity,
                        collider_type: hit_box_collider.collider_type,
                    },
                });
        }
    }
}
