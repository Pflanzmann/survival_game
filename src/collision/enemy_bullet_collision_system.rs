use bevy::prelude::{Entity, EventWriter, Query, Res, Transform, Vec2, With};

use crate::collision::SolidBodyCollisionQuadTreeHolder;
use crate::models::bullet::Bullet;
use crate::models::collider::collider_type::ColliderType;
use crate::models::collider::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::events::bullet_enemy_collision_event::BulletEnemyCollisionEvent;
use crate::util::quad_tree::QuadData;

pub fn enemy_bullet_collision_system(
    mut bullet_hit_event: EventWriter<BulletEnemyCollisionEvent>,
    quad_tree_holder: Res<SolidBodyCollisionQuadTreeHolder>,
    bullet_query: Query<(Entity, &Transform, &ColliderType), With<Bullet>>,
) {
    for (bullet_entity, bullet_transform, bullet_collider_type) in bullet_query.iter() {
        let mut check_entity_list: Vec<QuadData> = Vec::new();

        let size = match bullet_collider_type {
            Circle(radius) => Vec2::new(*radius, *radius),
            Rectangle(size) => *size,
        };

        quad_tree_holder.quad_tree.query_entities(
            &mut check_entity_list,
            &bullet_transform.translation,
            &size,
        );

        let mut colliding_entity: Option<Entity> = None;
        let mut last_distance: f32 = 1000000000.0;
        for quad_data in check_entity_list.iter() {
            if bullet_collider_type.is_colliding(&bullet_transform.translation.truncate(), &quad_data.collider_type, &quad_data.position.truncate()) {
                let current_distance = quad_data.position.distance(bullet_transform.translation);
                if current_distance < last_distance {
                    colliding_entity = Some(quad_data.entity);
                    last_distance = current_distance;
                }
            }
        }

        if let Some(entity) = colliding_entity {
            bullet_hit_event.send(BulletEnemyCollisionEvent {
                enemy_entity: entity,
                bullet_entity,
            })
        }
    }
}