use bevy::prelude::{Changed, Quat, Query, Transform, Vec3, With};

use crate::models::move_direction::MoveDirection;
use crate::models::sprite_rotate::SpriteRotate;

pub fn sprite_rotate_system(
    mut sprite_query: Query<(&MoveDirection, &mut Transform), (Changed<MoveDirection>, With<SpriteRotate>)>,
) {
    for (move_direction, mut transform) in sprite_query.iter_mut() {
        let target_pos = (transform.translation + move_direction.direction);
        let diff = target_pos - transform.translation;
        let angle = diff.y.atan2(diff.x);

        transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
    }
}
