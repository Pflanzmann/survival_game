use bevy::prelude::Entity;

pub struct EnemyCollisionEvent {
    pub target_entity: Entity,
    pub source_entity: Entity,
}