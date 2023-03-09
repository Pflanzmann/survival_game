use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct ApplyModToTargetEvent {
    pub mod_entity: Entity,
    pub target_entity: Entity,
}