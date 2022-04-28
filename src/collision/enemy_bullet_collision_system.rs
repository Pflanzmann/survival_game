use bevy::prelude::{Entity, EventWriter, Query, Res, Transform, Vec2, With};

use crate::collision::SolidBodyCollisionQuadTreeHolder;
use crate::models::bullet::Bullet;
use crate::models::collider::collider::Collider;
use crate::models::collider::collider_type::ColliderType;
use crate::models::collider::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::events::bullet_enemy_collision_event::BulletEnemyCollisionEvent;
use crate::util::quad_tree::QuadData;

pub fn enemy_bullet_collision_system(
    mut bullet_hit_event: EventWriter<BulletEnemyCollisionEvent>,
    quad_tree_holder: Res<SolidBodyCollisionQuadTreeHolder>,
    bullet_query: Query<(Entity, &Transform, &ColliderType), (With<Collider>, With<Bullet>)>,
) {
    for (bullet_entity, bullet_transform, bullet_size) in bullet_query.iter() {
        let mut check_entity_list: Vec<QuadData> = Vec::new();

        let size = match bullet_size {
            Circle(radius) => Vec2::new(*radius, *radius),
            Rectangle(size) => *size,
        };

        quad_tree_holder.quad_tree.query_entities(
            &mut check_entity_list,
            &bullet_transform.translation,
            &size,
        );

        let mut colliding_entities: Vec<Entity> = Vec::new();
        for quad_data in check_entity_list.iter() {
            if quad_data.collider_type.is_colliding(&quad_data.position.truncate(), bullet_size, &bullet_transform.translation.truncate()) {
                {
                    colliding_entities.push(quad_data.entity);
                }
            }

            if let Some(entity) = colliding_entities.first() {
                bullet_hit_event.send(BulletEnemyCollisionEvent {
                    enemy_entity: *entity,
                    bullet_entity,
                })
            }
        }
    }
}