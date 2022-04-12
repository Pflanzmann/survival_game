use bevy::prelude::Entity;

pub struct ApplyModToTargetSystem {
    pub mod_entity: Entity,
    pub target_entity: Entity,
}