use bevy::prelude::*;

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