use bevy::prelude::{Query, Res, Time};

use crate::models::collision::collided_entities::DamagedEntities;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage_interval::DamageInterval;

pub fn clear_damaged_entities_system(
    time: Res<Time>,
    mut query: Query<(&mut DamagedEntities, &DamageInterval)>,
) {
    for (mut damaged_entities, damage_interval) in query.iter_mut() {
        'inner: while let Some(damaged_entity) = damaged_entities.first() {
            if damaged_entity.damaged_time + (damage_interval.get_total_amount() as f64) < time.seconds_since_startup() {
                damaged_entities.remove(0);
            } else {
                break 'inner;
            }
        }
    }
}