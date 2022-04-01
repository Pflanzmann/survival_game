use bevy::prelude::{Commands, Entity, Query, With};
use bevy::sprite::collide_aabb::collide;

use crate::components::collision_components::{Collider, CollisionDirections};
use crate::components::unit_stats_components::{Enemy, UnitSize};
use crate::Transform;

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
            if collide(
                transform.translation,
                size.collider_size,
                other_transform.translation,
                other_size.collider_size,
            ).is_some() {
                let direction = (other_transform.translation - transform.translation).normalize();

                collisions.collisions.push((direction))
            }
        }
        commands.entity(entity).insert(collisions);
    }
}
