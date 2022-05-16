use bevy::prelude::{Changed, Query, Vec2};

use crate::models::animation::animation_state::{AnimationState, CurrentAnimationState};
use crate::models::move_direction::MoveDirection;

pub fn animation_state_handle_system(
    mut player_query: Query<(&mut CurrentAnimationState, &MoveDirection), Changed<MoveDirection>>
) {
    for (mut state, direction) in player_query.iter_mut() {
        if direction.direction == Vec2::new(0.0, 0.0) {
            state.state = AnimationState::Idle
        } else if direction.direction.x != 0.0 {
            state.state = AnimationState::WalkSide
        } else if direction.direction.y > 0.0 {
            state.state = AnimationState::WalkUp
        } else if direction.direction.y < 0.0 {
            state.state = AnimationState::WalkDown
        }
    }
}