use bevy::prelude::{Changed, Query, With};

use crate::models::animation::animation_state::{AnimationState, CurrentAnimationState};
use crate::models::animation::walking_animation_component::MoveAnimationDown;
use crate::models::move_direction::MoveDirection;

pub fn animation_move_down_state_handle_system(
    mut player_query: Query<(&mut CurrentAnimationState, &MoveDirection), (Changed<MoveDirection>, With<MoveAnimationDown>)>
) {
    for (mut state, direction) in player_query.iter_mut() {
        if direction.direction.y < 0.0 && direction.direction.y.abs() > direction.direction.x.abs() {
            state.state = AnimationState::WalkDown
        }
    }
}
