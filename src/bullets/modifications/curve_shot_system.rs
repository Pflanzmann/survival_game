use bevy::prelude::{Entity, Query, With};

use crate::entities::bullet_components::BulletRange;
use crate::entities::modification_components::CurveShot;
use crate::FacingDirection;

pub fn curve_shot_system(
    mut bullet_query: Query<(Entity, &mut FacingDirection, &BulletRange), With<CurveShot>>,
) {
    for (entity, mut direction, range) in bullet_query.iter_mut() {
        let angle_direction: f32 = if entity.id() as f32 % 2.0 == 0.0 { 1.0 } else { -1.0 };
        let x = direction.facing_direction.x;
        let y = direction.facing_direction.y;

        let angle = angle_direction * 0.000015 * range.distance_traveled;

        direction.facing_direction.x = x * angle.cos() - y * angle.sin();
        direction.facing_direction.y = x * angle.sin() + y * angle.cos();

        direction.facing_direction = direction.facing_direction.normalize();
    }
}