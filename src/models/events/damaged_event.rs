use bevy::prelude::Entity;

pub struct DamagedEvent {
    pub source_entity: Entity,
    pub target_entity: Entity,
}

impl DamagedEvent {
    pub fn new(source_entity: Entity, target_entity: Entity) -> Self {
        DamagedEvent { source_entity, target_entity }
    }
}
