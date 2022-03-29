use bevy::prelude::Transform;

use crate::{Query, With};
use crate::components::unit_stats_components::{Direction, MoveSpeed};
use crate::components::bullet_components::Bullet;

pub fn bullet_movement_system(
    mut bullet_query: Query<(&mut Transform, &Direction, &MoveSpeed), With<Bullet>>,
) {
    for (mut transform, direction, speed) in bullet_query.iter_mut() {
        transform.translation += direction.direction * speed.move_speed;
    }
}