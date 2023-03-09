use bevy::core::Name;
use bevy::prelude::{Commands, EventReader, Query, With};

use crate::models::bundles::damage_bundle::DamageBundle;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::mod_container_slot::ModContainerSlot;
use crate::models::modifications::acid_puddle::{AcidPuddle, AcidPuddleOwner};
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::weapon_slot::WeaponSlot;

pub fn apply_acid_puddle_system(
    mut commands: Commands,
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mod_query: Query<&AcidPuddle, With<Modification>>,
    holder_query: Query<&WeaponSlot>,
    gun_query: Query<&ModContainerSlot>,
) {
    for apply_event in apply_events.iter() {
        let projectile_mod = match mod_query.get(apply_event.mod_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        let holder_weapon_slot = match holder_query.get(apply_event.target_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        let weapon_mod_container = match gun_query.get(holder_weapon_slot.weapon_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        let acid_puddle_owner_entity = commands.spawn_empty()
            .insert(Name::new("AcidPuddleOwner"))
            .insert(DamageBundle::new(projectile_mod.damage, projectile_mod.damage_ticks_per_min))
            .id();

        commands.entity(weapon_mod_container.container_entity).insert(AcidPuddleOwner { owner: acid_puddle_owner_entity });
    }
}
