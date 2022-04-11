use crate::{Commands, Component, EventReader, Query, With};
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetSystem;
use crate::models::modification_attributes::modification::Modification;

pub fn apply_mod_to_target_system<T: Component + Clone>(
    mut commands: Commands,
    mut apply_events: EventReader<ApplyModToTargetSystem>,
    mod_query: Query<&T, With<Modification>>,
) {
    for apply_event in apply_events.iter() {
        let player_mod = match mod_query.get(apply_event.mod_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        commands.entity(apply_event.target_entity).insert(player_mod.clone());
    }
}