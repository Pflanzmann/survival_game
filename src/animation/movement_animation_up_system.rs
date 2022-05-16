use bevy::core::Time;
use bevy::prelude::*;

use crate::models::animation::animation_state::{AnimationState, CurrentAnimationState};
use crate::models::animation::walking_animation_component::MoveAnimationUp;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::move_speed::MoveSpeed;

pub fn movement_animation_up_system(
    time: Res<Time>,
    mut movers_query: Query<(&mut MoveAnimationUp, &CurrentAnimationState, &MoveSpeed, &mut TextureAtlasSprite)>,
) {
    for (mut animation_data, state, speed, mut sprite) in movers_query.iter_mut() {
        if !matches!(state.state, AnimationState::WalkUp) {
            continue;
        }

        animation_data.progress += time.delta_seconds() * speed.get_total_amount();
        sprite.index = (animation_data.progress as usize % animation_data.framecount) + (4 * animation_data.atlas_row);
    }
}

