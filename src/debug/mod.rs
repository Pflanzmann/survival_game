use bevy::prelude::{Plugin, SystemSet};

use crate::{App, ConsoleState};
use crate::debug::spawn_collision_boxes::{despawn_collision_boxes, init_collision_boxes, spawn_collision_boxes};
use crate::util::stage_label_helper::{in_collision, in_update};

mod spawn_collision_boxes;
pub mod show_quad_tree_system;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_update(
                    SystemSet::on_enter(ConsoleState::Shown)
                        .with_system(init_collision_boxes)
                )
            )

            .add_system_set(
                in_update(
                    SystemSet::on_update(ConsoleState::Shown)
                        .with_system(spawn_collision_boxes)
                )
            )

            .add_system_set(
                in_collision(
                    SystemSet::on_exit(ConsoleState::Shown)
                        .with_system(despawn_collision_boxes)
                )
            )
        ;
    }
}
