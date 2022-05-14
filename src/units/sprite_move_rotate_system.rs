use bevy::prelude::{Changed, Quat, Query, Transform, Vec3, With};

use crate::models::move_direction::MoveDirection;
use crate::models::sprite_move_rotation::SpriteMoveRotation;

pub fn sprite_move_rotate_system(
    mut sprite_query: Query<(&MoveDirection, &mut Transform), (Changed<MoveDirection>, With<SpriteMoveRotation>)>,
) {
    for (move_direction, mut transform) in sprite_query.iter_mut() {
        let angle = move_direction.direction.y.atan2(move_direction.direction.x);

        transform.rotation = Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle);
    }
}
