use bevy::prelude::{Query, Res, Time, Transform};

use crate::models::move_direction::MoveDirection;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::travel_range::TravelRange;

pub fn move_unit_system(
    time: Res<Time>,
    mut moving_entities: Query<(&mut Transform, &MoveDirection, &MoveSpeed, Option<&mut TravelRange>)>,
) {
    for (mut transform, direction, move_speed, travel_range) in moving_entities.iter_mut() {
        transform.translation.x += direction.direction.x * move_speed.get_total_amount() * time.delta_seconds() * 60.0;
        transform.translation.y += direction.direction.y * move_speed.get_total_amount() * time.delta_seconds() * 60.0;

        let mut travel_range = match travel_range {
            Some(value) => value,
            None => continue,
        };

        travel_range.distance_traveled += move_speed.get_total_amount() * time.delta_seconds() * 60.0
    }
}