use bevy::prelude::{EventReader, Query, With};

use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::mod_register::ModRegister;
use crate::models::modifications::descriptors::modification::Modification;

pub fn mod_list_deregister_system(
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mod_query: Query<With<Modification>>,
    mut target_query: Query<&mut ModRegister>,
) {
    for apply_event in apply_events.iter() {
        let modification = match mod_query.get(apply_event.mod_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        let mut target = match target_query.get_mut(apply_event.target_entity) {
            Ok(target) => target,
            Err(_) => continue,
        };

        target.register.remove(&apply_event.target_entity);
    }
}