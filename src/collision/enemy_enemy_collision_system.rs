use bevy::prelude::{Commands, Entity, Query, With};

use crate::components::collision_components::{Collider, CollisionDirections};
use crate::components::unit_stats_components::{Enemy, UnitSize};
use crate::Transform;
use crate::util::is_colliding::is_colliding;

pub fn enemy_enemy_collision_system(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform, &UnitSize), (With<Collider>, With<Enemy>)>,
) {
    let mut enemy_vec: Vec<(&Transform, &UnitSize)> = Vec::new();
    for (entity, transform, size) in enemy_query.iter() {
        enemy_vec.push((transform, size));
        commands.entity(entity).remove::<CollisionDirections>();
    }

    for (entity, transform, size) in enemy_query.iter() {
        let mut collisions = CollisionDirections { collisions: Vec::new() };

        for (other_entity, other_transform, other_size) in enemy_query.iter() {
            if is_colliding(
                transform.translation,
                size.collider_size,
                other_transform.translation,
                other_size.collider_size,
            ) {
                let mut direction = (other_transform.translation - transform.translation);

                if direction.length() != 0.0 {
                    direction = direction.normalize();
                }

                collisions.collisions.push((direction))
            }
        }
        commands.entity(entity).insert(collisions);
    }
}
