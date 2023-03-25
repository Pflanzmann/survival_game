use bevy::prelude::{Entity, Query, ResMut, Transform, Vec2, With};

use crate::models::collision::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::enemy::Enemy;
use crate::models::player::Player;
use crate::models::resources::collision::hit_box_quad_tree::{HitBoxData, HitBoxQuadTree};
use crate::util::quad_tree::{QuadData, Quadtree};

pub fn calculate_hit_box_quad_tree_system(
    mut hit_box_tree: ResMut<HitBoxQuadTree>,
    player_query: Query<&Transform, With<Player>>,
    hit_box_collider_query: Query<(Entity, &Transform, &HitBoxCollider), With<Enemy>>,
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
                    position: transform.translation.truncate(),
                    size,
                    data: HitBoxData {
                        entity,
                        collider_type: hit_box_collider.collider_type,
                    },
                });
        }
    }
}
