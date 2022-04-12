use bevy::prelude::{Entity, Query, With};

use crate::models::modification_components::CurveShot;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::models::unit_stats_components::MoveDirection;

pub fn curve_shot_system(
    mut bullet_query: Query<(Entity, &mut MoveDirection, &TravelRange), With<CurveShot>>,
) {
    for (entity, mut direction, range) in bullet_query.iter_mut() {
        let angle_direction: f32 = if entity.id() as f32 % 2.0 == 0.0 { 1.0 } else { -1.0 };
        let x = direction.direction.x;
        let y = direction.direction.y;

        let angle = angle_direction * 0.000015 * range.distance_traveled;

        direction.direction.x = x * angle.cos() - y * angle.sin();
        direction.direction.y = x * angle.sin() + y * angle.cos();

        direction.direction = direction.direction.normalize_or_zero();
    }
}