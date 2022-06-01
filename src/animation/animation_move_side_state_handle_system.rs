use bevy::prelude::{Changed, Query, With};

use crate::models::animation::animation_state::{AnimationState, CurrentAnimationState};
use crate::models::animation::walking_animation_component::MoveAnimationSide;
use crate::models::move_direction::MoveDirection;

pub fn animation_move_side_state_handle_system(
    mut player_query: Query<(&mut CurrentAnimationState, &MoveDirection), (Changed<MoveDirection>, With<MoveAnimationSide>)>
) {
    for (mut state, direction) in player_query.iter_mut() {
        if direction.direction.x != 0.0 {
            state.state = AnimationState::WalkSide
        }
    }
}