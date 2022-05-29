use bevy::prelude::{Query, Res};

use crate::models::damaged_entities::DamagedEntities;
use crate::models::resources::world::game_time::GameTime;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage_interval::DamageInterval;

pub fn clear_damaged_entities_system(
    game_time: Res<GameTime>,
    mut query: Query<(&mut DamagedEntities, &DamageInterval)>,
) {
    for (mut damaged_entities, damage_interval) in query.iter_mut() {
        'inner: while let Some(damaged_entity) = damaged_entities.first() {
            if damaged_entity.damaged_time + (damage_interval.get_total_amount() as f64) < game_time.time_in_seconds {
                damaged_entities.remove(0);
            } else {
                break 'inner;
            }
        }
    }
}