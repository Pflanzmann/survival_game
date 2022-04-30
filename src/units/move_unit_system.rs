use bevy::prelude::{Query, Res, Time, Transform};
use crate::models::behavior::steering_behavior::SteeringBehavior;

use crate::models::move_direction::MoveDirection;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::travel_range::TravelRange;

pub fn move_unit_system(
    time: Res<Time>,
    mut moving_entities: Query<(&mut Transform, &MoveDirection, &MoveSpeed, Option<&mut TravelRange>, Option<&SteeringBehavior>)>,
) {
    for (mut transform, direction, move_speed, travel_range, steering_behavior) in moving_entities.iter_mut() {
        let direction =  if let Some(steering_behavior) = steering_behavior {
            (direction.direction + steering_behavior.direction.extend(0.0)).normalize_or_zero()
        } else {
            direction.direction
        };

        transform.translation.x += direction.x * move_speed.get_total_amount() * time.delta_seconds() * 60.0;
        transform.translation.y += direction.y * move_speed.get_total_amount() * time.delta_seconds() * 60.0;


        if let Some(mut travel_range) = travel_range {
            travel_range.distance_traveled += move_speed.get_total_amount() * time.delta_seconds() * 60.0
        };
    }
}