use bevy::prelude::Transform;

use crate::{Query, With};
use crate::ai::ai_components::{Direction, Speed};
use crate::bullets::bullet_components::Bullet;

pub fn bullet_movement_system(
    mut bullet_query: Query<(&mut Transform, &Direction, &Speed), With<Bullet>>,
) {
    for (mut transform, direction, speed) in bullet_query.iter_mut() {
        transform.translation += direction.direction * speed.speed;
    }
}