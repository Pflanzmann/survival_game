use bevy::app::Plugin;
use bevy::prelude::{App, SystemSet};
use crate::AppState;
use crate::util::stage_label_helper::in_update;
use crate::animation::movement_animation_side_system::movement_animation_side_system;
use crate::animation::movement_animation_up_system::movement_animation_up_system;
use crate::animation::movement_animation_down_system::movement_animation_down_system;
use crate::animation::teleport_animation_system::teleport_animation_system;
use crate::animation::idle_animation_system::idle_animation_system;
use crate::animation::animation_state_handle_system::animation_state_handle_system;
use crate::animation::fade_animation_system::fade_animation_system;

mod movement_animation_side_system;
mod idle_animation_system;
mod animation_state_handle_system;
mod movement_animation_down_system;
mod movement_animation_up_system;
mod teleport_animation_system;
mod fade_animation_system;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_update(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(movement_animation_side_system)
                        .with_system(movement_animation_up_system)
                        .with_system(movement_animation_down_system)
                        .with_system(idle_animation_system)
                        .with_system(animation_state_handle_system)
                        .with_system(teleport_animation_system)
                        .with_system(fade_animation_system)
                )
            );
    }
}

