use bevy::ecs::component::Component;
use bevy::prelude::{Deref, DerefMut, Entity};

#[derive(Component, Default, Clone, Deref, DerefMut)]
pub struct DamagedEntities(Vec<DamagedEntity>);

#[derive(Clone)]
pub struct DamagedEntity {
    pub entity: Entity,
    pub damaged_time: f64,
}

impl DamagedEntity {
    pub fn new(entity: Entity, damaged_time: f64) -> Self {
        DamagedEntity { entity, damaged_time }
    }
}

impl PartialEq<Self> for DamagedEntity {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}
