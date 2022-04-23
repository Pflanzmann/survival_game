use std::time::SystemTime;

use bevy::prelude::{Commands, Entity, Query, Res, ResMut, Transform, With};

use crate::collision::quad_tree::QuadData;
use crate::collision::QuadTreeHolder;
use crate::models::collider::collider::Collider;
use crate::models::collider::collision_directions::CollisionDirections;
use crate::models::enemy::Enemy;
use crate::models::unit_size::UnitSize;
use crate::util::is_colliding::is_colliding;

pub fn enemy_enemy_collision_system(
    mut commands: Commands,
    quad_tree_holder: Res<QuadTreeHolder>,
    enemy_query: Query<(Entity, &Transform, &UnitSize), (With<Collider>, With<Enemy>)>,
) {
    let start_time = SystemTime::now();

    for (entity, _, _) in enemy_query.iter() {
        commands.entity(entity).remove::<CollisionDirections>();
    }

    let mut counter = 0;
    let mut max_comp = 0;
    for (entity, transform, size) in enemy_query.iter() {
        let mut collisions = CollisionDirections { collisions: Vec::new() };

        let mut check_entity_list: Vec<QuadData> = Vec::new();
        quad_tree_holder.quad_tree.query_entities(
            &mut check_entity_list,
            &transform.translation.truncate(),
            &size.collider_size,
        );

        if max_comp < check_entity_list.len() {
            max_comp = check_entity_list.len();
        }

        for other_entity in check_entity_list.iter() {
            counter += 1;
            if is_colliding(
                transform.translation,
                size.collider_size,
                other_entity.position,
                other_entity.size,
            ) {
                let mut direction = other_entity.position - transform.translation;

                direction = direction.normalize_or_zero();

                collisions.collisions.push(direction);
            }
        }
        commands.entity(entity).insert(collisions);
    }

    let end = SystemTime::now();
    println!("new system time: {:?}, count: {counter} | max query: {max_comp}", end.duration_since(start_time));
}
