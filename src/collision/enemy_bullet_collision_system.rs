use bevy::prelude::{Entity, EventWriter, GlobalTransform, Query, Res, Vec2, With};

use crate::models::bullet::Bullet;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::collider_type::ColliderType::{Circle, Rectangle};
use crate::models::events::bullet_enemy_collision_event::BulletEnemyCollisionEvent;
use crate::models::resources::solid_body_quad_tree::{SolidBodyData, SolidBodyQuadTree};
use crate::util::quad_tree::QuadData;

pub fn enemy_bullet_collision_system(
    mut bullet_hit_event: EventWriter<BulletEnemyCollisionEvent>,
    quad_tree_holder: Res<SolidBodyQuadTree>,
    bullet_query: Query<(Entity, &GlobalTransform, &ColliderType), With<Bullet>>,
) {
    for (bullet_entity, bullet_transform, bullet_collider_type) in bullet_query.iter() {
        let mut check_entity_list: Vec<QuadData<SolidBodyData>> = Vec::new();

        let size = match bullet_collider_type {
            Circle(radius) => Vec2::new(*radius, *radius),
            Rectangle(size) => *size,
        };

        quad_tree_holder.query_entities(
            &mut check_entity_list,
            &bullet_transform.translation,
            &size,
        );

        let mut colliding_entities: Vec<Entity> = Vec::new();
        for quad_data in check_entity_list.iter() {
            if bullet_collider_type.is_colliding(&bullet_transform.translation.truncate(), &quad_data.data.collider_type, &quad_data.position.truncate()) {
                colliding_entities.push(quad_data.data.entity);
            }
        }

        for colliding_entity in colliding_entities.iter() {
            bullet_hit_event.send(BulletEnemyCollisionEvent {
                enemy_entity: *colliding_entity,
                bullet_entity,
            })
        }
    }
}