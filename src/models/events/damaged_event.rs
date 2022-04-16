use bevy::prelude::Entity;

pub struct DamagedEvent {
    pub damaged_entity: Entity,
}

impl DamagedEvent {
    pub fn new(damaged_entity: Entity) -> Self {
        Self { damaged_entity}
    }
}
