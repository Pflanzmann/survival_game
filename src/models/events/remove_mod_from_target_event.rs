use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct RemoveModFromTargetEvent {
    pub mod_entity: Entity,
    pub target_entity: Entity,
}