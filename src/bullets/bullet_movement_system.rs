use bevy::prelude::{Commands, Transform};

use crate::{Entity, Query, With};
use crate::components::bullet_components::{Bullet, BulletRange};
use crate::components::unit_stats_components::{FacingDirection, MoveSpeed};

pub fn bullet_movement_system(
    mut commands: Commands,
    mut bullet_query: Query<(&mut Transform, &FacingDirection, &MoveSpeed, &mut BulletRange, Entity), With<Bullet>>,
) {
    for (mut transform, direction, speed, mut bullet_range, entity) in bullet_query.iter_mut() {
        let distance_to_move = speed.move_speed;
        bullet_range.distance_traveled += distance_to_move;

        if bullet_range.distance_traveled >= bullet_range.total_range {
            commands.entity(entity).despawn();
            continue;
        }

        transform.translation += direction.facing_direction * distance_to_move;
    }
}