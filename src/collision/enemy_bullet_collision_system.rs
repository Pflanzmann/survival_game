use bevy::prelude::{Entity, EventWriter, Query, Res, Transform, With};

use crate::collision::SolidBodyCollisionQuadTreeHolder;
use crate::util::quad_tree::QuadData;
use crate::models::bullet::Bullet;
use crate::models::collider::collider::Collider;
use crate::models::events::bullet_enemy_collision_event::BulletEnemyCollisionEvent;
use crate::models::unit_size::UnitSize;
use crate::util::is_colliding::is_colliding;

pub fn enemy_bullet_collision_system(
    mut bullet_hit_event: EventWriter<BulletEnemyCollisionEvent>,
    quad_tree_holder: Res<SolidBodyCollisionQuadTreeHolder>,
    bullet_query: Query<(Entity, &Transform, &UnitSize), (With<Collider>, With<Bullet>)>,
) {
    for (bullet_entity, bullet_transform, bullet_size) in bullet_query.iter() {
        let mut check_entity_list: Vec<QuadData> = Vec::new();
        quad_tree_holder.quad_tree.query_entities(
            &mut check_entity_list,
            &bullet_transform.translation,
            &bullet_size.collider_size,
        );

        let mut colliding_entities: Vec<Entity> = Vec::new();
        for quad_data in check_entity_list.iter() {
            if is_colliding(
                quad_data.position,
                quad_data.size,
                bullet_transform.translation,
                bullet_size.collider_size,
            ) {
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
