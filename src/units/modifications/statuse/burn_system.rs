use bevy::prelude::{Commands, Entity, EventWriter, Query, Res, Time};

use crate::models::events::damaged_event::DamagedEvent;
use crate::models::modifications::statuses::burning::Burning;
use crate::models::unit_attributes::health::Health;

pub fn burn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut damaged_event: EventWriter<DamagedEvent>,
    mut burning_query: Query<(Entity, &mut Burning, &mut Health)>,
) {
    for (target_entity, mut burning, mut health) in burning_query.iter_mut() {
        burning.debuff_duration -= time.delta_seconds();
        burning.damage_interval_timer -= time.delta_seconds();

        if burning.damage_interval_timer < 0.0 {
            burning.damage_interval_timer = burning.damage_interval;

            health.damage(burning.damage_per_tick);
            damaged_event.send(DamagedEvent::new(burning.source_entity, target_entity))
        }

        if burning.debuff_duration < 0.0 {
            commands.entity(target_entity).remove::<Burning>();
        }
    }
}