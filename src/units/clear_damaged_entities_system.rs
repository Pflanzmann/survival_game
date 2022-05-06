use bevy::prelude::{Query, Res, Time};

use crate::models::collision::collided_entities::DamagedEntities;

pub fn clear_damaged_entities_system(
    time: Res<Time>,
    mut query: Query<&mut DamagedEntities>,
) {
    for mut damaged_entities in query.iter_mut() {
        'inner: while let Some(damaged_entity) = damaged_entities.first() {
            if damaged_entity.damaged_time + 1.0 < time.seconds_since_startup() {
                damaged_entities.remove(0);
            } else {
                break 'inner;
            }
        }
    }
}