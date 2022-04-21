use std::time::SystemTime;

use bevy::prelude::{Commands, Entity, Query, ResMut, Transform, With};

use crate::collision::QuadTreeHolder;
use crate::models::collider::collider::Collider;
use crate::models::collider::collision_directions::CollisionDirections;
use crate::models::enemy::Enemy;
use crate::models::unit_size::UnitSize;
use crate::util::is_colliding::is_colliding;

pub fn enemy_enemy_collision_system_old(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform, &UnitSize), (With<Collider>, With<Enemy>)>,
) {
    let start_time = SystemTime::now();
    for (entity, _, _) in enemy_query.iter() {
        commands.entity(entity).remove::<CollisionDirections>();
    }

    let mut counter = 0;
    for (entity, transform, size) in enemy_query.iter() {
        let mut collisions = CollisionDirections { collisions: Vec::new() };

        for (other_entity, other_transform, other_size) in enemy_query.iter() {
            counter += 1;
            if is_colliding(
                transform.translation,
                size.collider_size,
                other_transform.translation,
                other_size.collider_size,
            ) {
                let mut direction = other_transform.translation - transform.translation;

                direction = direction.normalize_or_zero();

                collisions.collisions.push(direction);
            }
        }
        commands.entity(entity).insert(collisions);
    }

    let end = SystemTime::now();
    println!("old system time: {:?}, count: {}", end.duration_since(start_time), counter);
}
