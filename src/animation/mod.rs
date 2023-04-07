use bevy::app::Plugin;
use bevy::prelude::*;

use crate::animation::animation_move_down_state_handle_system::animation_move_down_state_handle_system;
use crate::animation::animation_move_idle_state_handle_system::animation_move_idle_state_handle_system;
use crate::animation::animation_move_side_state_handle_system::animation_move_side_state_handle_system;
use crate::animation::animation_move_up_state_handle_system::animation_move_up_state_handle_system;
use crate::animation::fade_animation_system::{fade_animation_sprite_system, fade_animation_texture_atlas_system};
use crate::animation::idle_animation_system::idle_animation_system;
use crate::animation::movement_animation_down_system::movement_animation_down_system;
use crate::animation::movement_animation_side_system::movement_animation_side_system;
use crate::animation::movement_animation_up_system::movement_animation_up_system;
use crate::animation::teleport_animation_system::teleport_animation_system;
use crate::AppState;
use crate::scheduling::BaseSets;

mod movement_animation_side_system;
mod idle_animation_system;
mod movement_animation_down_system;
mod movement_animation_up_system;
mod teleport_animation_system;
mod fade_animation_system;
mod animation_move_side_state_handle_system;
mod animation_move_down_state_handle_system;
mod animation_move_up_state_handle_system;
mod animation_move_idle_state_handle_system;

pub struct AnimationPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct AnimationSystemSet;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            AnimationSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(in_state(AppState::InGame))
        );

        app
            .add_system(movement_animation_side_system.in_set(AnimationSystemSet))
            .add_system(movement_animation_up_system.in_set(AnimationSystemSet))
            .add_system(movement_animation_down_system.in_set(AnimationSystemSet))
            .add_system(idle_animation_system.in_set(AnimationSystemSet))
            .add_system(teleport_animation_system.in_set(AnimationSystemSet))
            .add_system(fade_animation_sprite_system.in_set(AnimationSystemSet))
            .add_system(fade_animation_texture_atlas_system.in_set(AnimationSystemSet))
            .add_system(animation_move_idle_state_handle_system.in_set(AnimationSystemSet))
            .add_system(animation_move_down_state_handle_system.in_set(AnimationSystemSet))
            .add_system(animation_move_side_state_handle_system.in_set(AnimationSystemSet))
            .add_system(animation_move_up_state_handle_system.in_set(AnimationSystemSet));
    }
}

