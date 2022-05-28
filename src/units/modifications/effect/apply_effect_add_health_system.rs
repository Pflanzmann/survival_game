use bevy::prelude::{EventReader, Query};

use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::modifications::effects::effect_add_health::EffectAddHealth;
use crate::models::unit_attributes::health::Health;

pub fn apply_effect_add_health_system(
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mut target_query: Query<&mut Health>,
    effect_query: Query<&EffectAddHealth>,
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

        target_health.heal(effect.amount);
    }
}
