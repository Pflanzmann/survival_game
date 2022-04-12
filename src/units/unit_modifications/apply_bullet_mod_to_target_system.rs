use bevy::prelude::{Commands, Component, EventReader, Query, With, Without};

use crate::models::events::apply_mod_to_target_event::ApplyModToTargetSystem;
use crate::models::mod_container_slot::ModContainerSlot;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::player::Player;
use crate::models::weapon_slot::WeaponSlot;

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