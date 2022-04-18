use bevy::prelude::{Query, With};

use crate::models::aim_direction::AimDirection;
use crate::models::behavior::spin_aim_behavior::SpinAimBehavior;

pub fn spin_aim_behavior_system(
    mut unit_query: Query<&mut AimDirection, With<SpinAimBehavior>>,
) {
    for mut unit_aim_direction in unit_query.iter_mut() {
        let mut direction = unit_aim_direction.direction;
        let x = direction.x;
        let y = direction.y;

        let angle: f32 = 0.1;

        direction.x = x * angle.cos() - y * angle.sin();
        direction.y = x * angle.sin() + y * angle.cos();

        unit_aim_direction.direction = direction.normalize_or_zero();
    }
}