use bevy::prelude::*;

/// This plugin controls scheduling between different [ game stages ].
/// These are used to control globally which systems are running at any given time.
///
/// [ first_label_system ], [collision_label_system], [pre_update_label_system], [update_label_system] and [post_update_label_system]
/// serve as simulated Stage boundary to help schedule the system runtime
///
/// [execute_state_switch_system] performs a switch between [ game stages ] in a controlled environment
/// to guaranty execution time in the beginning of a game tick and avoid overlapping calls for state changes
///
/// For each state there is an [ on_enter ] event that serves as a kind of startup
/// system to each state.
pub struct SchedulingPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
pub enum BaseSets {
    First,
    Collision,
    PreUpdate,
    Update,
    PostUpdate,
    Last,
    Flush,
}

impl Plugin for SchedulingPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_sets((
                CoreSet::UpdateFlush,
                BaseSets::First,
                BaseSets::Collision,
                BaseSets::PreUpdate,
                BaseSets::Update,
                BaseSets::PostUpdate,
                BaseSets::Last,
                BaseSets::Flush,
                CoreSet::PostUpdate
            ).chain())

            .add_system(apply_system_buffers.in_base_set(BaseSets::Flush))
        ;
    }
}