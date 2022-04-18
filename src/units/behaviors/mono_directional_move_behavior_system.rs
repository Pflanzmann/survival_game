use bevy::prelude::Query;

use crate::models::behavior::mono_directional_move_behavior::MonoDirectionalMoveBehavior;
use crate::models::move_direction::MoveDirection;

pub fn mono_directional_move_behavior_system(
    mut actor_query: Query<(&mut MoveDirection, &MonoDirectionalMoveBehavior)>
) {
    for (mut move_direction, mono_dir_move_behavior) in actor_query.iter_mut() {
        move_direction.direction = mono_dir_move_behavior.direction
    };
}