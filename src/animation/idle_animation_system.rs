use bevy::core::Time;
use bevy::prelude::*;

use crate::models::animation::animation_state::{AnimationState, CurrentAnimationState};
use crate::models::animation::idle_animation_component::IdleAnimation;
use crate::SPRITE_ROW_LENGTH;

pub fn idle_animation_system(
    time: Res<Time>,
    mut movers_query: Query<(&mut IdleAnimation, &CurrentAnimationState, &mut TextureAtlasSprite)>,
) {
    for (mut animation_data, state, mut sprite) in movers_query.iter_mut() {
        if !matches!(state.state, AnimationState::Idle) {
            continue;
        }

        animation_data.progress += (time.delta_seconds() * animation_data.animation_frame_count as f32) / animation_data.duration;
        sprite.index = (animation_data.progress as usize % animation_data.animation_frame_count) + (SPRITE_ROW_LENGTH * animation_data.atlas_row);
    }
}

