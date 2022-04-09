use bevy::prelude::{Commands, Entity, Name, Query, With};

use crate::{Gunnable, Player};
use crate::models::gun_components::{Reloadable, StraightBasicShot, WeaponSlot};
use crate::models::modification_components::{ModContainer, ModContainerSlot};

pub fn setup_gun_system(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    for entity in player_query.iter() {
        let mod_container = commands.spawn()
            .insert(Name::new("gun mod container"))
            .insert(ModContainer)
            .id();

        let gun = commands.spawn()
            .insert(Name::new("basic gun"))
            .insert(Gunnable)
            .insert(StraightBasicShot)
            .insert(Reloadable::new(2.0))
            .insert(ModContainerSlot { container_entity: mod_container })
            .id();

        commands.entity(entity).insert(WeaponSlot { weapon_entity: gun });
    }
}