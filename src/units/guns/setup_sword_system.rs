use bevy::prelude::{Added, BuildChildren, Commands, Entity, Name, Query, Res};

use crate::assets_handling::preload_projectile_system::ProjectileConfigHandles;
use crate::models::attribute_container::AttributeContainer;
use crate::models::attribute_container_slot::AttributeContainerSlot;
use crate::models::gun::basic_sword::BasicSword;
use crate::models::mod_container::ModContainer;
use crate::models::mod_container_slot::ModContainerSlot;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::models::weapon_slot::WeaponSlot;

pub fn setup_sword_system(
    mut commands: Commands,
    target_query: Query<Entity, Added<BasicSword>>,
    projectile_handle: Res<ProjectileConfigHandles>,
) {
    for target_entity in target_query.iter() {
        let mod_container = commands.spawn_empty()
            .insert(Name::new("BasicSword ModContainer"))
            .insert(ModContainer)
            .id();

        let attribute_container = commands.spawn_empty()
            .insert(Name::new("BasicSword AttributeContainer"))
            .insert(AttributeContainer)
            .insert(Damage::new(projectile_handle.basic_projectile.damage))
            .insert(TravelRange::new(projectile_handle.basic_projectile.range))
            .id();

        let gun_entity = commands.spawn_empty()
            .insert(Name::new("BasicSword"))
            .insert(ModContainerSlot { container_entity: mod_container })
            .insert(AttributeContainerSlot { container_entity: attribute_container })
            .id();

        commands.entity(gun_entity).add_child(attribute_container);
        commands.entity(gun_entity).add_child(mod_container);
        commands.entity(target_entity).insert(WeaponSlot { weapon_entity: gun_entity });
    }
}
