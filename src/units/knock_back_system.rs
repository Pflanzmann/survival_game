use bevy::prelude::{Commands, EventReader, Query};

use crate::models::events::damaged_event::DamagedEvent;
use crate::models::knock_back::KnockBack;
use crate::models::move_direction::MoveDirection;
use crate::models::unit_push::UnitPush;

pub fn knock_back_system(
    mut commands: Commands,
    mut hit_event: EventReader<DamagedEvent>,
    source_query: Query<(&MoveDirection, &KnockBack)>,
) {
    for event in hit_event.iter() {
        let (move_direction, knock_back) = match source_query.get(event.source_entity) {
            Ok(source) => source,
            Err(_) => continue,
        };

        commands.entity(event.target_entity).insert(UnitPush {
            direction: move_direction.direction,
            duration: knock_back.push_duration,
            force: knock_back.push_force,
        });
    }
}