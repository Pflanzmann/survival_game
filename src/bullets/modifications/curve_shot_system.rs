use bevy::prelude::Query;

use crate::components::modification_components::CurveShot;
use crate::FacingDirection;

pub fn curve_shot_system(
    mut bullet_query: Query<(&mut FacingDirection, &CurveShot)>,
) {
    for (mut direction, curve_shot) in bullet_query.iter_mut() {
        let angle: f32 = if curve_shot.curve_left { 0.005 } else { -0.005 };
        let x = direction.facing_direction.x;
        let y = direction.facing_direction.y;

        direction.facing_direction.x = x * angle.cos() - y * angle.sin();
        direction.facing_direction.y = x * angle.sin() + y * angle.cos();

        direction.facing_direction = direction.facing_direction.normalize();
    }
}