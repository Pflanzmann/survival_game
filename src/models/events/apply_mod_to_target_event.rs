use bevy::prelude::Entity;

pub struct ApplyModToTargetEvent {
    pub mod_entity: Entity,
    pub target_entity: Entity,
}