use bevy::prelude::{Commands, Entity, Name, Query, With};
use crate::models::mod_container::ModContainer;

use crate::models::straight_basic_shot::StraightBasicShot;
use crate::models::mod_container_slot::ModContainerSlot;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::reload::Reload;
use crate::models::weapon_slot::WeaponSlot;
use crate::models::player::Player;

pub fn setup_gun_system(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    for player_entity in player_query.iter() {
        let mod_container = commands.spawn()
            .insert(Name::new("BasicGun ModContainer"))
            .insert(ModContainer)
            .id();

        let gun_entity = commands.spawn()
            .insert(Name::new("BasicGun"))
            .insert(StraightBasicShot)
            .insert(ModContainerSlot { container_entity: mod_container })
            .id();

        commands.entity(player_entity).insert(WeaponSlot { weapon_entity: gun_entity });
        commands.entity(player_entity).insert(Reload::new(1.0));
    }
}
