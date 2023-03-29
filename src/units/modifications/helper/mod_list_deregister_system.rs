use bevy::prelude::{EventReader, Query, With};

use crate::models::events::remove_mod_from_target_event::RemoveModFromTargetEvent;
use crate::models::mod_register::ModRegister;
use crate::models::modifications::descriptors::modification::Modification;

pub fn mod_list_deregister_system(
    mut apply_events: EventReader<RemoveModFromTargetEvent>,
    mod_query: Query<With<Modification>>,
    mut target_query: Query<&mut ModRegister>,
) {
    for remove_event in apply_events.iter() {
        if mod_query.get(remove_event.mod_entity).is_err() {
            continue;
        };

        let mut target = match target_query.get_mut(remove_event.target_entity) {
            Ok(target) => target,
            Err(_) => continue,
        };

        target.register.retain(|element| {
            return !element.eq(&remove_event.mod_entity);
        });
    }
}