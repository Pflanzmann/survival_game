use bevy::prelude::{Commands, Entity, Query};
use bevy::sprite::collide_aabb::collide;

use crate::ai::ai_components::Size;
use crate::collision::collision_components::Collider;
use crate::Transform;

pub fn collision_detection_system(
    mut commands: Commands,
    collider_query: Query<(Entity, &Transform, &Collider, &Size)>,
) {
    let mut test_objects = Vec::<(Entity, &Transform, &Collider, &Size)>::new();

    for (test_entity, test_transform, test_collider, test_size) in collider_query.iter() {
        test_objects.push((test_entity, test_transform, test_collider, test_size))
    }

    for (pos, (test_entity, test_transform, test_collider, test_size)) in test_objects.iter().enumerate() {
        if pos + 1 < test_objects.len() - 1 {
            for n in pos..(test_objects.len() - 1) {
                let (other_entity, other_transform, other_collider, other_size) = test_objects[n];

                if collide(
                    test_transform.translation,
                    test_size.size,
                    other_transform.translation,
                    other_size.size,
                ).is_some() {
//                    commands.entity(other_entity).insert
                }
            }
        }
    }
}