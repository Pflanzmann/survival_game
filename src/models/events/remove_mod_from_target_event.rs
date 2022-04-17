use bevy::prelude::Entity;

pub struct RemoveModFromTargetEvent {
    pub mod_entity: Entity,
    pub target_entity: Entity,
}