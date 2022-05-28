use bevy::prelude::{EventReader, EventWriter, Query};

use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::events::damaged_event::DamagedEvent;
use crate::models::modifications::effects::effect_damage_health::EffectDamageHealth;
use crate::models::unit_attributes::health::Health;

pub fn apply_effect_damage_health_system(
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mut damaged_event: EventWriter<DamagedEvent>,
    mut target_query: Query<&mut Health>,
    effect_query: Query<&EffectDamageHealth>,
) {
    for apply_event in apply_events.iter() {
        let effect = match effect_query.get(apply_event.mod_entity) {
            Ok(effect) => effect,
            Err(_) => continue,
        };

        let mut target_health = match target_query.get_mut(apply_event.target_entity) {
            Ok(health) => health,
            Err(_) => continue,
        };

        target_health.damage(effect.amount);
        damaged_event.send(DamagedEvent {
            source_entity: apply_event.mod_entity,
            target_entity: apply_event.target_entity,
        });
    }
}
