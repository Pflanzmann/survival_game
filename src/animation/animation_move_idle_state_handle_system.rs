use bevy::prelude::{Changed, Query, Vec2, With};

use crate::models::animation::animation_state::{AnimationState, CurrentAnimationState};
use crate::models::animation::idle_animation_component::IdleAnimation;
use crate::models::move_direction::MoveDirection;

pub fn animation_move_idle_state_handle_system(
    mut player_query: Query<(&mut CurrentAnimationState, &MoveDirection), (Changed<MoveDirection>, With<IdleAnimation>)>
) {
    for (mut state, direction) in player_query.iter_mut() {
        if direction.direction == Vec2::new(0.0, 0.0) {
            state.state = AnimationState::Idle
        }
    }
}