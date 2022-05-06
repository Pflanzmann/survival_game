use bevy::core::Time;
use bevy::prelude::*;

use crate::models::animation::animation_state::{AnimationState, CurrentAnimationState};
use crate::models::animation::idle_animation_component::IdleAnimation;

pub fn idle_animation_system(
    time: Res<Time>,
    mut movers_query: Query<(&mut IdleAnimation, &CurrentAnimationState, &mut TextureAtlasSprite)>,
) {
    for (mut animation_data, state, mut sprite) in movers_query.iter_mut() {
        if !matches!(state.state, AnimationState::Idle) {
            continue;
        }

        animation_data.progress += time.delta_seconds() * 2.0;
        sprite.index = (animation_data.progress as usize % animation_data.framecount) + (4 * animation_data.atlas_row);
    }
}

