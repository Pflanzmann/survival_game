use bevy::prelude::Without;

use crate::{Commands, Component, EventReader, Player, Query, With};
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetSystem;
use crate::models::gun_components::WeaponSlot;
use crate::models::modification_attributes::modification::Modification;
use crate::models::modification_components::ModContainerSlot;

pub fn apply_bullet_mod_to_target_system<T: Component + Clone>(
    mut commands: Commands,
    mut apply_events: EventReader<ApplyModToTargetSystem>,
    mod_query: Query<&T, With<Modification>>,
    player_query: Query<&WeaponSlot, With<Player>>,
    gun_query: Query<&ModContainerSlot, Without<Player>>,
) {
    for apply_event in apply_events.iter() {
        let bullet_mod = match mod_query.get(apply_event.mod_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        let player_weapon_slot = match player_query.get(apply_event.target_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        let weapon_mod_container = match gun_query.get(player_weapon_slot.weapon_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        commands.entity(weapon_mod_container.container_entity).insert(bullet_mod.clone());
    }
}