use bevy::prelude::{Commands, EventReader, Query, With};

use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::mod_register::ModRegister;
use crate::models::modifications::descriptors::modification::Modification;

pub fn mod_list_register_system(
    mut commands: Commands,
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mod_query: Query<With<Modification>>,
    mut target_query: Query<Option<&mut ModRegister>>,
) {
    for apply_event in apply_events.iter() {
        if mod_query.get(apply_event.mod_entity).is_err() {
            continue;
        };

        let target = match target_query.get_mut(apply_event.target_entity) {
            Ok(target) => target,
            Err(_) => continue,
        };

        if let Some(mut register) = target {
            if register.register.contains(&apply_event.mod_entity) {
                return;
            }

            register.register.insert(0, apply_event.mod_entity);
        } else {
            let mut register = ModRegister::default();
            register.register.push(apply_event.mod_entity);

            commands.entity(apply_event.target_entity).insert(register);
        }
    }
}