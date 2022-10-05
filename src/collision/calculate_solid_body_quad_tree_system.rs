use bevy::prelude::{Entity, GlobalTransform, Query, ResMut, Vec2, With};

use crate::models::collision::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::collision::collider_weight::ColliderWeight;
use crate::models::collision::solid_body_collider::SolidBodyCollider;
use crate::models::player::Player;
use crate::models::resources::collision::solid_body_quad_tree::{SolidBodyData, SolidBodyQuadTree};
use crate::util::quad_tree::{QuadData, Quadtree};

pub fn calculate_solid_body_quad_tree_system(
    mut solid_body_tree: ResMut<SolidBodyQuadTree>,
    player_query: Query<&GlobalTransform, With<Player>>,
    solid_body_query: Query<(Entity, &GlobalTransform, &SolidBodyCollider, &ColliderWeight)>,
) {
    for player_position in player_query.iter() {
        let quad_position = player_position.translation().truncate();

        solid_body_tree.0 = Quadtree::new(10000.0, 10000.0, quad_position, 0);
        for (entity, transform, solid_body_collider, collision_weight) in solid_body_query.iter() {
            let size: Vec2 = match solid_body_collider.collider_type {
                Circle(radius) => Vec2::new(radius, radius),
                Rectangle(size) => size,
            };

            solid_body_tree.insert(
                &QuadData {
                    position: transform.translation().truncate() + solid_body_collider.offset,
                    size,
                    data: SolidBodyData {
                        entity,
                        collider_type: solid_body_collider.collider_type,
                        collision_weight: collision_weight.weight,
                    },
                });
        }
    }
}
