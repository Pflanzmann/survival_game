use bevy::prelude::{Changed, Transform, Quat, Query, Vec3, With, GlobalTransform};

use crate::models::aim_direction::AimDirection;
use crate::models::sprite_aim_rotation::SpriteAimRotation;

pub fn sprite_aim_rotate_system(
    mut sprite_query: Query<(&AimDirection, &mut Transform), (Changed<AimDirection>, With<SpriteAimRotation>)>,
) {
    for (aim_direction, mut transform) in sprite_query.iter_mut() {
        let target_pos = transform.translation + aim_direction.direction;
        let diff = target_pos - transform.translation;
        let angle = diff.y.atan2(diff.x);

        transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
    }
}
