use bevy::prelude::{Commands, Entity, Query, Res, Transform, With};

use crate::collision::EnemyCollisionQuadTreeHolder;
use crate::models::collider::collider::Collider;
use crate::models::collider::collision_directions::CollisionDirections;
use crate::models::enemy::Enemy;
use crate::models::unit_size::UnitSize;
use crate::util::is_colliding::is_colliding;
use crate::util::quad_tree::QuadData;

pub fn enemy_enemy_collision_system(
    mut commands: Commands,
    quad_tree_holder: Res<EnemyCollisionQuadTreeHolder>,
    mut enemy_query: Query<(Entity, &mut Transform, &UnitSize), (With<Collider>, With<Enemy>)>,
) {
    for (entity, _, _) in enemy_query.iter() {
        commands.entity(entity).remove::<CollisionDirections>();
    }

    for (entity, mut transform, size) in enemy_query.iter_mut() {
        let mut collisions = CollisionDirections { collisions: Vec::new() };

        let mut check_entity_list: Vec<QuadData> = Vec::new();
        quad_tree_holder.quad_tree.query_entities(
            &mut check_entity_list,
            &transform.translation,
            &size.collider_size,
        );

        for quad_data in check_entity_list.iter() {
            if quad_data.entity == entity {
                continue;
            }

            if is_colliding(
                transform.translation,
                size.collider_size,
                quad_data.position,
                quad_data.size,
            ) {
                let mut direction = quad_data.position - transform.translation;

                direction = direction.normalize_or_zero();

                collisions.collisions.push(direction);
            }
        }
        commands.entity(entity).insert(collisions);
    }
}
