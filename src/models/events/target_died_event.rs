use bevy::prelude::Entity;

pub struct TargetDiedEvent {
    pub target_entity: Entity,
}
