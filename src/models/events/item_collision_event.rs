use bevy::prelude::Entity;

pub struct ItemCollisionEvent {
    pub source_entity: Entity,
    pub target_entity: Entity,
}

