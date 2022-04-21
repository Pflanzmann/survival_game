use std::time::SystemTime;
use bevy::prelude::{Entity, Query, ResMut, Transform, Vec2, With};

use crate::collision::quad_tree::Quadtree;
use crate::collision::QuadTreeHolder;
use crate::models::collider::collider::Collider;
use crate::models::enemy::Enemy;
use crate::models::player::Player;
use crate::models::unit_size::UnitSize;

pub fn calculate_quad_tree_system(
    mut quad_tree_holder: ResMut<QuadTreeHolder>,
    player_query: Query<&Transform, With<Player>>,
    entity_query: Query<(Entity, &Transform, &UnitSize), (With<Collider>, With<Enemy>)>,
) {
    let start_time = SystemTime::now();
    let player_position = match player_query.get_single() {
        Ok(transform) => transform.translation,
        Err(_) => return,
    };

    let quad_position = player_position.truncate();

    quad_tree_holder.quad_tree = Quadtree::new(5000.0, 5000.0, quad_position, 0);
    let mut counter = 0;
    for (entity, transform, size) in entity_query.iter() {
        quad_tree_holder.quad_tree.insert(
            &entity,
            &transform.translation.truncate(),
            &size.collider_size,
        );
        counter += 1;
    }

    let end = SystemTime::now();
    println!("calc tree time: {:?} | depth: {}", end.duration_since(start_time), quad_tree_holder.quad_tree.print_structure());
}