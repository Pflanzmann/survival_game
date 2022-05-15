use bevy::ecs::bundle::Bundle;

use crate::models::damaged_entities::DamagedEntities;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::damage_interval::DamageInterval;

#[derive(Bundle)]
pub struct DamageBundle {
    damage: Damage,
    damage_interval: DamageInterval,
    damaged_entities: DamagedEntities,
}

impl DamageBundle {
    pub fn new(base_damage: f32, base_damage_interval: f32) -> Self {
        DamageBundle {
            damage: Damage::new(base_damage),
            damage_interval: DamageInterval::new(base_damage_interval),
            damaged_entities: DamagedEntities::default(),
        }
    }
}

