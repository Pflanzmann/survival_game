use bevy::prelude::{Entity, Query, ResMut, Transform, With, Without};

use crate::models::bullet::Bullet;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::collision_weight::CollisionWeight;
use crate::models::items::descriptor::item::Item;
use crate::models::player::Player;
use crate::models::resources::item_collision_quad_tree::ItemCollisionQuadTree;
use crate::models::resources::solid_body_collision_quad_tree::SolidBodyCollisionQuadTree;
use crate::util::quad_tree::{QuadData, Quadtree};

pub fn calculate_quad_tree_system(
    mut enemy_quad_tree_holder: ResMut<SolidBodyCollisionQuadTree>,
    mut item_tree_holder: ResMut<ItemCollisionQuadTree>,
    player_query: Query<&Transform, With<Player>>,
    entity_query: Query<(Entity, &Transform, &ColliderType, &CollisionWeight), (Without<Bullet>, Without<Item>)>,
    item_query: Query<(Entity, &Transform, &ColliderType), With<Item>>,
) {
    let player_position = match player_query.get_single() {
        Ok(transform) => transform.translation,
        Err(_) => return,
    };

    let quad_position = player_position.truncate();

    enemy_quad_tree_holder.0 = Quadtree::new(10000.0, 10000.0, quad_position, 0);
    for (entity, transform, collider_type, collision_weight) in entity_query.iter() {
        enemy_quad_tree_holder.insert(
            &QuadData {
                entity,
                position: transform.translation,
                collider_type: *collider_type,
                collision_weight: collision_weight.weight,
            });
    }

    item_tree_holder.0 = Quadtree::new(10000.0, 10000.0, quad_position, 0);
    for (entity, transform, collider_type) in item_query.iter() {
        item_tree_holder.insert(
            &QuadData {
                entity,
                position: transform.translation,
                collider_type: *collider_type,
                collision_weight: 0.0,
            });
    }
}