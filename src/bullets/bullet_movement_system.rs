use bevy::prelude::Transform;

use crate::{Query, With};
use crate::components::unit_stats_components::{FacingDirection, MoveSpeed};
use crate::components::bullet_components::Bullet;

pub fn bullet_movement_system(
    mut bullet_query: Query<(&mut Transform, &FacingDirection, &MoveSpeed), With<Bullet>>,
) {
    for (mut transform, direction, speed) in bullet_query.iter_mut() {
        transform.translation += direction.facing_direction * speed.move_speed;
    }
}