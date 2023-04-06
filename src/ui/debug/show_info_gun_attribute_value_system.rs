use std::any::type_name;

use bevy::prelude::*;

use crate::models::attribute_container::AttributeContainer;
use crate::models::attribute_container_slot::AttributeContainerSlot;
use crate::models::player::Player;
use crate::models::resources::ui_states::info_window_state::InfoWindowState;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::weapon_slot::WeaponSlot;

pub fn update_info_gun_attribute_value_system<
    T: Component + Attribute,
>(
    weapon_owner_query: Query<&WeaponSlot, With<Player>>,
    source_query: Query<&AttributeContainerSlot>,
    attribute_container_query: Query<&T, With<AttributeContainer>>,
    mut state: ResMut<InfoWindowState>,
) {
    for weapon_slot in weapon_owner_query.iter() {
        let attribute_container_slot = match source_query.get(weapon_slot.weapon_entity) {
            Ok(source) => source,
            Err(_) => continue,
        };

        let attribute = match attribute_container_query.get(attribute_container_slot.container_entity) {
            Ok(source) => source,
            Err(_) => continue,
        };

        let text = attribute.get_total_amount().to_string();

        state.infos.insert(format!("Gun {}", type_name::<T>().split("::").last().unwrap().to_string()), text);
    }
}