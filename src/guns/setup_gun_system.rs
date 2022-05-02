use bevy::prelude::{Commands, Entity, Name, Query, Res, With};

use crate::assets_handling::preload_bullet_system::BulletConfigHandles;
use crate::assets_handling::preload_player_system::PlayerConfigHandles;
use crate::models::attribute_container::AttributeContainer;
use crate::models::attribute_container_slot::AttributeContainerSlot;
use crate::models::mod_container::ModContainer;
use crate::models::mod_container_slot::ModContainerSlot;
use crate::models::player::Player;
use crate::models::straight_basic_shot::StraightBasicShot;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::hit_limit::HitLimit;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::reload::Reload;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::models::weapon_slot::WeaponSlot;

pub fn setup_gun_system(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    player_handles: Res<PlayerConfigHandles>,
    bullet_handle: Res<BulletConfigHandles>,
) {
    for player_entity in player_query.iter() {
        let mod_container = commands.spawn()
            .insert(Name::new("BasicGun ModContainer"))
            .insert(ModContainer)
            .id();

        let attribute_container = commands.spawn()
            .insert(Name::new("BasicGun AttributeContainer"))
            .insert(AttributeContainer)
            .insert(Damage::new(bullet_handle.basic_bullet.damage))
            .insert(HitLimit::new(bullet_handle.basic_bullet.hit_limit))
            .insert(MoveSpeed::new(bullet_handle.basic_bullet.speed))
            .insert(TravelRange::new(bullet_handle.basic_bullet.range))
            .id();

        let gun_entity = commands.spawn()
            .insert(Name::new("BasicGun"))
            .insert(StraightBasicShot)
            .insert(ModContainerSlot { container_entity: mod_container })
            .insert(AttributeContainerSlot { container_entity: attribute_container })
            .id();

        commands.entity(player_entity).insert(WeaponSlot { weapon_entity: gun_entity });
        commands.entity(player_entity).insert(Reload::new(player_handles.player_one.reload));
    }
}
