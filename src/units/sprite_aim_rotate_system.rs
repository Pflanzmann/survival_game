use bevy::prelude::{Changed, Quat, Query, Transform, Vec3, With};

use crate::models::aim_direction::AimDirection;
use crate::models::sprite_aim_rotation::SpriteAimRotation;

pub fn sprite_aim_rotate_system(
    mut sprite_query: Query<(&AimDirection, &mut Transform), (Changed<AimDirection>, With<SpriteAimRotation>)>,
) {
    for (aim_direction, mut transform) in sprite_query.iter_mut() {
        let target_pos = transform.translation.truncate() + aim_direction.direction;
        let diff = target_pos - transform.translation.truncate();
        let angle = diff.y.atan2(diff.x);

        transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
    }
}
