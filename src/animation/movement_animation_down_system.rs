use bevy::prelude::*;

use crate::models::animation::animation_state::{AnimationState, CurrentAnimationState};
use crate::models::animation::walking_animation_component::MoveAnimationDown;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::SPRITE_ROW_LENGTH;

pub fn movement_animation_down_system(
    time: Res<Time>,
    mut movers_query: Query<(&mut MoveAnimationDown, &CurrentAnimationState, &MoveSpeed, &mut TextureAtlasSprite)>,
) {
    for (mut animation_data, state, speed, mut sprite) in movers_query.iter_mut() {
        if !matches!(state.state, AnimationState::WalkDown) {
            continue;
        }

        animation_data.progress += time.delta_seconds() * speed.get_total_amount();
        sprite.index = (animation_data.progress as usize % animation_data.framecount) + (SPRITE_ROW_LENGTH * animation_data.atlas_row);
    }
}

