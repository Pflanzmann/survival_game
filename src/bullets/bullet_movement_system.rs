use bevy::prelude::Transform;

use crate::{Query, With};
use crate::components::unit_stats_components::{Direction, Speed};
use crate::components::bullet_components::Bullet;

pub fn bullet_movement_system(
    mut bullet_query: Query<(&mut Transform, &Direction, &Speed), With<Bullet>>,
) {
    for (mut transform, direction, speed) in bullet_query.iter_mut() {
        transform.translation += direction.direction * speed.speed;
    }
}