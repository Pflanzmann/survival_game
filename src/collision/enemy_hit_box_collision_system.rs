use bevy::prelude::{Entity, EventWriter, GlobalTransform, Query, Res, Vec2, With};

use crate::models::collision::collider_owner::ColliderOwner;
use crate::models::collision::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::collision::enemy_hit_box_collider::EnemyHitBoxCollider;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::events::enemy_collision_event::EnemyCollisionEvent;
use crate::models::resources::collision::hit_box_quad_tree::{HitBoxData, HitBoxQuadTree};
use crate::util::quad_tree::QuadData;

pub fn enemy_hit_box_collision_system(
    mut enemy_collision_event: EventWriter<EnemyCollisionEvent>,
    quad_tree: Res<HitBoxQuadTree>,
    source_query: Query<(Entity, &GlobalTransform, &HitBoxCollider, Option<&ColliderOwner>), With<EnemyHitBoxCollider>>,
) {
    for (entity, transform, hit_box_collider, collision_owner) in source_query.iter() {
        let mut check_entity_list: Vec<QuadData<HitBoxData>> = Vec::new();

        let entity = match collision_owner {
            Some(owner) => *(*owner),
            None => entity,
        };

        let size = match hit_box_collider.collider_type {
            Circle(radius) => Vec2::new(radius, radius),
            Rectangle(size) => size,
        };

        quad_tree.query_entities(
            &mut check_entity_list,
            &transform.translation().truncate(),
            &size,
        );

        for quad_data in check_entity_list.iter() {
            if hit_box_collider.collider_type.is_colliding(
                &transform.translation().truncate(),
                &quad_data.data.collider_type,
                &quad_data.position,
            ) {
                enemy_collision_event.send(EnemyCollisionEvent {
                    target_entity: quad_data.data.entity,
                    source_entity: entity,
                })
            }
        }
    }
}
