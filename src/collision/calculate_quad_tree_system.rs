use bevy::prelude::{Entity, Query, ResMut, Transform, With};

use crate::collision::{EnemyCollisionQuadTreeHolder, ItemCollisionQuadTreeHolder};
use crate::util::quad_tree::{QuadData, Quadtree};
use crate::models::collider::collider::Collider;
use crate::models::enemy::Enemy;
use crate::models::items::descriptor::item::Item;
use crate::models::player::Player;
use crate::models::unit_size::UnitSize;

pub fn calculate_quad_tree_system(
    mut enemy_quad_tree_holder: ResMut<EnemyCollisionQuadTreeHolder>,
    mut item_tree_holder: ResMut<ItemCollisionQuadTreeHolder>,
    player_query: Query<&Transform, With<Player>>,
    entity_query: Query<(Entity, &Transform, &UnitSize), (With<Collider>, With<Enemy>)>,
    item_query: Query<(Entity, &Transform, &UnitSize), (With<Collider>, With<Item>)>,
) {
    let player_position = match player_query.get_single() {
        Ok(transform) => transform.translation,
        Err(_) => return,
    };

    let quad_position = player_position.truncate();

    enemy_quad_tree_holder.quad_tree = Quadtree::new(10000.0, 10000.0, quad_position, 0);
    for (entity, transform, size) in entity_query.iter() {
        enemy_quad_tree_holder.quad_tree.insert(
            &QuadData {
                entity,
                position: transform.translation,
                size: size.collider_size,
            });
    }

    item_tree_holder.quad_tree = Quadtree::new(10000.0, 10000.0, quad_position, 0);
    for (entity, transform, size) in item_query.iter() {
        item_tree_holder.quad_tree.insert(
            &QuadData {
                entity,
                position: transform.translation,
                size: size.collider_size,
            });
    }
}