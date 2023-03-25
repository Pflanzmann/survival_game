use bevy::prelude::{Entity, EventWriter, Query, Res, Transform, Vec2, With};

use crate::models::collision::collider_owner::ColliderOwner;
use crate::models::collision::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::collision::enemy_solid_body_collider::EnemySolidBodyCollider;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::events::enemy_collision_event::EnemyCollisionEvent;
use crate::models::resources::collision::solid_body_quad_tree::{SolidBodyData, SolidBodyQuadTree};
use crate::util::quad_tree::QuadData;

pub fn enemy_solid_body_collision_system(
    mut enemy_hit_event: EventWriter<EnemyCollisionEvent>,
    quad_tree: Res<SolidBodyQuadTree>,
    source_query: Query<(Entity, &Transform, &HitBoxCollider, Option<&ColliderOwner>), With<EnemySolidBodyCollider>>,
) {
    for (entity, transform, hit_box_collider, collision_owner) in source_query.iter() {
        let mut check_entity_list: Vec<QuadData<SolidBodyData>> = Vec::new();

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
            &transform.translation.truncate(),
            &size,
        );

        for quad_data in check_entity_list.iter() {
            if hit_box_collider.collider_type.is_colliding(
                &transform.translation.truncate(),
                &quad_data.data.collider_type,
                &quad_data.position,
            ) {
                enemy_hit_event.send(EnemyCollisionEvent {
                    target_entity: quad_data.data.entity,
                    source_entity: entity,
                })
            }
        }
    }
}